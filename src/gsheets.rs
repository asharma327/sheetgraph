//! Google Sheets backend: auth (service account, gcloud ADC user creds, or a
//! raw token), chunked parallel fetch of grid data, conversion to the model.

use crate::a1::{col_name, quote_sheet, Rect, OPEN};
use crate::model::*;
use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use serde_json::Value as J;
use std::collections::HashMap;
use std::sync::{mpsc, Mutex};
use std::time::{Duration, Instant};

const SHEETS: &str = "https://sheets.googleapis.com/v4/spreadsheets";
const SCOPES: &str = "https://www.googleapis.com/auth/spreadsheets.readonly https://www.googleapis.com/auth/drive.readonly";
/// Cells requested per API call. Full cell data is ~150-300 bytes/cell of JSON.
const CELLS_PER_CHUNK: u32 = 120_000;
const WORKERS: usize = 4;

// ---------------------------------------------------------------- auth

enum Cred {
    ServiceAccount { email: String, key_pem: String },
    User { client_id: String, client_secret: String, refresh_token: String },
    Static(String),
}

pub struct Auth {
    http: reqwest::blocking::Client,
    cred: Cred,
    cached: Mutex<Option<(String, Instant)>>,
}

impl Auth {
    /// Resolution order: explicit path, $SHEETGRAPH_TOKEN, $GOOGLE_APPLICATION_CREDENTIALS,
    /// gcloud application-default credentials.
    pub fn discover(path: Option<&str>) -> Result<Auth> {
        let http = reqwest::blocking::Client::builder().timeout(Duration::from_secs(300)).build()?;
        if path.is_none()
            && let Ok(t) = std::env::var("SHEETGRAPH_TOKEN") {
                return Ok(Auth { http, cred: Cred::Static(t), cached: Mutex::new(None) });
            }
        let p = match path {
            Some(p) => p.to_string(),
            None => match std::env::var("GOOGLE_APPLICATION_CREDENTIALS") {
                Ok(p) => p,
                // Where `gcloud auth application-default login` writes them.
                Err(_) if cfg!(windows) => {
                    let appdata = std::env::var("APPDATA").unwrap_or_default();
                    format!("{appdata}\\gcloud\\application_default_credentials.json")
                }
                Err(_) => {
                    let home = std::env::var("HOME").unwrap_or_default();
                    format!("{home}/.config/gcloud/application_default_credentials.json")
                }
            },
        };
        let j: J = serde_json::from_str(&std::fs::read_to_string(&p).with_context(|| format!("reading credentials {p}"))?)?;
        let s = |k: &str| j[k].as_str().unwrap_or_default().to_string();
        let cred = match j["type"].as_str() {
            Some("service_account") => Cred::ServiceAccount { email: s("client_email"), key_pem: s("private_key") },
            Some("authorized_user") => Cred::User {
                client_id: s("client_id"),
                client_secret: s("client_secret"),
                refresh_token: s("refresh_token"),
            },
            other => bail!("unsupported credential type {other:?} in {p}"),
        };
        Ok(Auth { http, cred, cached: Mutex::new(None) })
    }

    pub fn identity(&self) -> String {
        match &self.cred {
            Cred::ServiceAccount { email, .. } => email.clone(),
            Cred::User { .. } => "gcloud user credentials".into(),
            Cred::Static(_) => "$SHEETGRAPH_TOKEN".into(),
        }
    }

    pub fn token(&self) -> Result<String> {
        let mut c = self.cached.lock().unwrap();
        if let Some((t, exp)) = c.as_ref()
            && Instant::now() < *exp {
                return Ok(t.clone());
            }
        let resp: J = match &self.cred {
            Cred::Static(t) => return Ok(t.clone()),
            Cred::ServiceAccount { email, key_pem } => {
                #[derive(serde::Serialize)]
                struct Claims<'a> {
                    iss: &'a str,
                    scope: &'a str,
                    aud: &'a str,
                    iat: u64,
                    exp: u64,
                }
                let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();
                let claims = Claims { iss: email, scope: SCOPES, aud: "https://oauth2.googleapis.com/token", iat: now, exp: now + 3600 };
                let key = jsonwebtoken::EncodingKey::from_rsa_pem(key_pem.as_bytes())?;
                let jwt = jsonwebtoken::encode(&jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256), &claims, &key)?;
                self.http
                    .post("https://oauth2.googleapis.com/token")
                    .form(&[("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"), ("assertion", jwt.as_str())])
                    .send()?
                    .json()?
            }
            Cred::User { client_id, client_secret, refresh_token } => self
                .http
                .post("https://oauth2.googleapis.com/token")
                .form(&[
                    ("grant_type", "refresh_token"),
                    ("client_id", client_id.as_str()),
                    ("client_secret", client_secret.as_str()),
                    ("refresh_token", refresh_token.as_str()),
                ])
                .send()?
                .json()?,
        };
        let t = resp["access_token"].as_str().ok_or_else(|| anyhow!("token exchange failed: {resp}"))?.to_string();
        let ttl = resp["expires_in"].as_u64().unwrap_or(3600).saturating_sub(120);
        *c = Some((t.clone(), Instant::now() + Duration::from_secs(ttl)));
        Ok(t)
    }

    fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<String> {
        let mut wait = Duration::from_millis(1000);
        for attempt in 0..7 {
            let r = self.http.get(url).query(query).bearer_auth(self.token()?).send();
            match r {
                Ok(r) if r.status().is_success() => return Ok(r.text()?),
                Ok(r) => {
                    let st = r.status();
                    let body = r.text().unwrap_or_default();
                    if !(st.as_u16() == 429 || st.is_server_error()) || attempt == 6 {
                        bail!("GET {url} → {st}: {}", body.chars().take(400).collect::<String>());
                    }
                }
                Err(e) if attempt == 6 => return Err(e.into()),
                Err(_) => {}
            }
            std::thread::sleep(wait);
            wait = (wait * 2).min(Duration::from_secs(60));
        }
        unreachable!()
    }
}

// ---------------------------------------------------------------- API shapes

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct GridData {
    start_row: u32,
    start_column: u32,
    row_data: Vec<RowData>,
    row_metadata: Vec<DimMeta>,
    column_metadata: Vec<DimMeta>,
}
#[derive(Deserialize, Default)]
#[serde(default)]
struct RowData {
    values: Vec<CellData>,
}
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct DimMeta {
    hidden_by_user: bool,
}
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct CellData {
    user_entered_value: Option<ExtValue>,
    effective_value: Option<ExtValue>,
    formatted_value: Option<String>,
    user_entered_format: Option<J>,
    data_validation: Option<J>,
    note: Option<String>,
    hyperlink: Option<String>,
    pivot_table: Option<J>,
}
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct ExtValue {
    number_value: Option<f64>,
    string_value: Option<String>,
    bool_value: Option<bool>,
    formula_value: Option<String>,
    error_value: Option<ErrVal>,
}
#[derive(Deserialize, Default)]
#[serde(default)]
struct ErrVal {
    #[serde(rename = "type")]
    kind: String,
    message: String,
}

impl ExtValue {
    fn to_value(&self) -> Value {
        if let Some(n) = self.number_value {
            Value::Num(n)
        } else if let Some(s) = &self.string_value {
            Value::Str(s.clone())
        } else if let Some(b) = self.bool_value {
            Value::Bool(b)
        } else if let Some(e) = &self.error_value {
            let code = match e.kind.as_str() {
                "DIVIDE_BY_ZERO" => "#DIV/0!",
                "REF" => "#REF!",
                "NAME" => "#NAME?",
                "NUM" => "#NUM!",
                "N_A" => "#N/A",
                "VALUE" => "#VALUE!",
                "NULL_VALUE" => "#NULL!",
                "LOADING" => "#LOADING",
                _ => "#ERROR!",
            };
            let msg = e.message.chars().take(80).collect::<String>();
            Value::Err { err: if msg.is_empty() { code.into() } else { format!("{code} {msg}") } }
        } else {
            Value::Empty
        }
    }
}

// ---------------------------------------------------------------- fetch

pub fn fetch(auth: &Auth, id_or_url: &str, verbose: bool) -> Result<Workbook> {
    let id = crate::formula::spreadsheet_id(id_or_url);
    let meta_fields = "spreadsheetId,spreadsheetUrl,properties(title,locale,timeZone),namedRanges,dataSources,developerMetadata,\
        sheets(properties,merges,conditionalFormats,protectedRanges,basicFilter,filterViews,charts,bandedRanges,developerMetadata)";
    let meta: J = serde_json::from_str(&auth.get(&format!("{SHEETS}/{id}"), &[("fields", meta_fields)])?)?;

    let mut wb = Workbook {
        source: Source {
            kind: "gsheet".into(),
            id: id.clone(),
            title: meta["properties"]["title"].as_str().unwrap_or_default().into(),
            locale: meta["properties"]["locale"].as_str().map(Into::into),
            time_zone: meta["properties"]["timeZone"].as_str().map(Into::into),
            url: meta["spreadsheetUrl"].as_str().map(Into::into),
            ..Default::default()
        },
        ..Default::default()
    };
    // Drive metadata is best-effort: needs the drive scope and visibility.
    match auth.get(
        &format!("https://www.googleapis.com/drive/v3/files/{id}"),
        &[("fields", "modifiedTime,owners(emailAddress),lastModifyingUser(emailAddress)"), ("supportsAllDrives", "true")],
    ) {
        Ok(t) => {
            let d: J = serde_json::from_str(&t).unwrap_or_default();
            wb.source.modified = d["modifiedTime"].as_str().map(Into::into);
            wb.source.owner = d["owners"][0]["emailAddress"].as_str().map(Into::into);
        }
        Err(e) => wb.warnings.push(format!("drive metadata unavailable: {}", short(&e.to_string()))),
    }

    let sheets_j = meta["sheets"].as_array().cloned().unwrap_or_default();
    let mut id_to_name: HashMap<i64, String> = HashMap::new();
    for s in &sheets_j {
        let p = &s["properties"];
        id_to_name.insert(p["sheetId"].as_i64().unwrap_or(0), p["title"].as_str().unwrap_or_default().into());
    }
    let gr = |g: &J| grid_range(g);
    let mut styles = Interner::<Style>::default();
    let mut dvs = Interner::<Validation>::default();

    for (i, s) in sheets_j.iter().enumerate() {
        let p = &s["properties"];
        let gp = &p["gridProperties"];
        let mut sh = Sheet {
            name: p["title"].as_str().unwrap_or_default().into(),
            index: i,
            hidden: p["hidden"].as_bool().unwrap_or(false),
            kind: p["sheetType"].as_str().unwrap_or("GRID").into(),
            grid_rows: gp["rowCount"].as_u64().unwrap_or(0) as u32,
            grid_cols: gp["columnCount"].as_u64().unwrap_or(0) as u32,
            frozen_rows: gp["frozenRowCount"].as_u64().unwrap_or(0) as u32,
            frozen_cols: gp["frozenColumnCount"].as_u64().unwrap_or(0) as u32,
            tab_color: color_hex(&p["tabColorStyle"]["rgbColor"]).or_else(|| color_hex(&p["tabColor"])),
            ..Default::default()
        };
        for m in arr(&s["merges"]) {
            sh.merges.push(gr(m));
        }
        for cf in arr(&s["conditionalFormats"]) {
            sh.cond_formats.push(CondFormat { ranges: arr(&cf["ranges"]).iter().map(gr).collect(), rule: cond_rule(cf) });
        }
        if s["basicFilter"].is_object() {
            sh.filters.push(filter(&s["basicFilter"], None));
        }
        for fv in arr(&s["filterViews"]) {
            sh.filters.push(filter(fv, fv["title"].as_str().map(Into::into)));
        }
        for pr in arr(&s["protectedRanges"]) {
            sh.protected.push(Protected {
                range: pr.get("range").map(gr),
                description: pr["description"].as_str().map(Into::into),
                warning_only: pr["warningOnly"].as_bool().unwrap_or(false),
            });
        }
        for b in arr(&s["bandedRanges"]) {
            sh.banded.push(gr(&b["range"]));
        }
        for c in arr(&s["charts"]) {
            sh.charts.push(chart(c, &id_to_name));
        }
        for d in arr(&s["developerMetadata"]) {
            wb.developer_metadata.push(d.clone());
        }
        wb.sheets.push(sh);
    }
    for nr in arr(&meta["namedRanges"]) {
        let r = &nr["range"];
        let sheet = id_to_name.get(&r["sheetId"].as_i64().unwrap_or(0)).cloned().unwrap_or_default();
        wb.named_ranges.push(NamedRange {
            name: nr["name"].as_str().unwrap_or_default().into(),
            range: format!("{}!{}", quote_sheet(&sheet), grid_range(r)),
        });
    }
    wb.data_sources = arr(&meta["dataSources"]).to_vec();
    wb.developer_metadata.extend(arr(&meta["developerMetadata"]).iter().cloned());

    // Plan chunks over every grid sheet.
    let mut jobs: Vec<(usize, u32, u32)> = Vec::new();
    for (i, sh) in wb.sheets.iter().enumerate() {
        if sh.kind != "GRID" || sh.grid_rows == 0 || sh.grid_cols == 0 {
            continue;
        }
        let per = (CELLS_PER_CHUNK / sh.grid_cols).max(20);
        let mut r = 0;
        while r < sh.grid_rows {
            let r1 = (r + per).min(sh.grid_rows) - 1;
            jobs.push((i, r, r1));
            r = r1 + 1;
        }
    }
    let total = jobs.len();
    let fields = "sheets(data(startRow,startColumn,rowMetadata(hiddenByUser),columnMetadata(hiddenByUser),\
        rowData(values(userEnteredValue,effectiveValue,formattedValue,userEnteredFormat,dataValidation,note,hyperlink,pivotTable))))";
    let names: Vec<(String, u32)> = wb.sheets.iter().map(|s| (s.name.clone(), s.grid_cols)).collect();
    let queue = Mutex::new(jobs.into_iter());
    let (tx, rx) = mpsc::channel::<Result<(usize, u32, GridData)>>();
    let t0 = Instant::now();

    std::thread::scope(|scope| -> Result<()> {
        for _ in 0..WORKERS {
            let tx = tx.clone();
            let queue = &queue;
            let names = &names;
            let id = &id;
            scope.spawn(move || loop {
                let Some((si, r0, r1)) = queue.lock().unwrap().next() else { break };
                let (name, cols) = &names[si];
                let range = format!("{}!A{}:{}{}", quote_sheet(name), r0 + 1, col_name(cols - 1), r1 + 1);
                let res = auth
                    .get(&format!("{SHEETS}/{id}"), &[("ranges", range.as_str()), ("fields", fields)])
                    .and_then(|body| {
                        let mut v: J = serde_json::from_str(&body)?;
                        let gd = v["sheets"][0]["data"][0].take();
                        if gd.is_null() {
                            return Ok(GridData::default());
                        }
                        Ok(serde_json::from_value::<GridData>(gd)?)
                    })
                    .with_context(|| format!("fetching {range}"));
                if tx.send(res.map(|g| (si, r0, g))).is_err() {
                    break;
                }
            });
        }
        drop(tx);
        let mut done = 0;
        for res in rx {
            let (si, r0, gd) = res?;
            done += 1;
            if verbose {
                eprint!("\r  {} chunks {done}/{total} ({:.0}s)   ", wb.source.title, t0.elapsed().as_secs_f32());
            }
            absorb(&mut wb.sheets[si], r0, gd, &mut styles, &mut dvs, &id_to_name);
        }
        if verbose && total > 0 {
            eprintln!();
        }
        Ok(())
    })?;

    for sh in &mut wb.sheets {
        sh.cells.sort_by_key(|c| (c.r, c.c));
        sh.hidden_rows = merge_spans(std::mem::take(&mut sh.hidden_rows));
    }
    wb.styles = styles.items;
    wb.validations = dvs.items;
    canonicalize(&mut wb);
    wb.warnings.push(
        "Apps Script (bound scripts, macros, add-ons, triggers) is not exposed by the Sheets API; \
         pass --apps-script <scriptId> with user credentials to include a project"
            .into(),
    );
    Ok(wb)
}

/// Fetch an Apps Script project's source (needs user OAuth with script.projects.readonly).
pub fn fetch_apps_script(auth: &Auth, script_id: &str) -> Result<Vec<Script>> {
    let body = auth.get(&format!("https://script.googleapis.com/v1/projects/{script_id}/content"), &[])?;
    let j: J = serde_json::from_str(&body)?;
    Ok(arr(&j["files"])
        .iter()
        .map(|f| Script {
            kind: "apps_script".into(),
            name: format!("{}.{}", f["name"].as_str().unwrap_or("?"), f["type"].as_str().unwrap_or("").to_ascii_lowercase()),
            source: f["source"].as_str().unwrap_or_default().into(),
        })
        .collect())
}

fn absorb(
    sh: &mut Sheet,
    _r0: u32,
    gd: GridData,
    styles: &mut Interner<Style>,
    dvs: &mut Interner<Validation>,
    id_to_name: &HashMap<i64, String>,
) {
    let (sr, sc) = (gd.start_row, gd.start_column);
    for (i, m) in gd.row_metadata.iter().enumerate() {
        if m.hidden_by_user {
            push_span(&mut sh.hidden_rows, sr + i as u32);
        }
    }
    if sh.hidden_cols.is_empty() && sr == 0 {
        for (i, m) in gd.column_metadata.iter().enumerate() {
            if m.hidden_by_user {
                push_span(&mut sh.hidden_cols, sc + i as u32);
            }
        }
    }
    for (ri, row) in gd.row_data.into_iter().enumerate() {
        let r = sr + ri as u32;
        for (ci, cd) in row.values.into_iter().enumerate() {
            let c = sc + ci as u32;
            if let Some(p) = &cd.pivot_table {
                sh.pivots.push(pivot(p, (r, c), id_to_name));
            }
            let f = cd.user_entered_value.as_ref().and_then(|u| u.formula_value.clone());
            let v = match (&cd.effective_value, &cd.user_entered_value) {
                (Some(e), _) => e.to_value(),
                (None, Some(u)) if f.is_none() => u.to_value(),
                _ => Value::Empty,
            };
            let t = match (&v, &cd.formatted_value) {
                (Value::Num(n), Some(fv)) if *fv != fmt_num(*n) => Some(fv.clone()),
                _ => None,
            };
            let s = cd.user_entered_format.as_ref().map(style_of).filter(|s| *s != Style::default()).map(|s| styles.intern(s));
            let dv = cd.data_validation.as_ref().map(|d| dvs.intern(validation(d)));
            let cell = Cell { r, c, v, f, t, s, dv, note: cd.note, link: cd.hyperlink };
            if cell.is_blank() && cell.s.is_none() && cell.dv.is_none() && cell.note.is_none() {
                continue;
            }
            sh.cells.push(cell);
        }
    }
}

fn merge_spans(mut v: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    v.sort();
    let mut out: Vec<(u32, u32)> = Vec::new();
    for (a, b) in v {
        match out.last_mut() {
            Some(l) if l.1 + 1 >= a => l.1 = l.1.max(b),
            _ => out.push((a, b)),
        }
    }
    out
}

fn push_span(spans: &mut Vec<(u32, u32)>, i: u32) {
    if let Some(last) = spans.last_mut()
        && last.1 + 1 == i {
            last.1 = i;
            return;
        }
    spans.push((i, i));
}

// ---------------------------------------------------------------- conversions

fn arr(j: &J) -> &[J] {
    j.as_array().map(|v| v.as_slice()).unwrap_or(&[])
}

fn short(s: &str) -> String {
    s.chars().take(160).collect()
}

pub fn grid_range(g: &J) -> Rect {
    let n = |k: &str| g[k].as_u64().map(|v| v as u32);
    Rect {
        r0: n("startRowIndex").unwrap_or(0),
        c0: n("startColumnIndex").unwrap_or(0),
        r1: n("endRowIndex").map(|v| v.saturating_sub(1)).unwrap_or(OPEN),
        c1: n("endColumnIndex").map(|v| v.saturating_sub(1)).unwrap_or(OPEN),
    }
}

fn grid_range_a1(g: &J, id_to_name: &HashMap<i64, String>) -> String {
    let sheet = id_to_name.get(&g["sheetId"].as_i64().unwrap_or(0)).cloned().unwrap_or_default();
    format!("{}!{}", quote_sheet(&sheet), grid_range(g))
}

fn color_hex(c: &J) -> Option<String> {
    if !c.is_object() {
        return None;
    }
    let ch = |k: &str| (c[k].as_f64().unwrap_or(0.0) * 255.0).round() as u8;
    Some(format!("#{:02X}{:02X}{:02X}", ch("red"), ch("green"), ch("blue")))
}

fn color_of(style: &J, plain: &J) -> Option<String> {
    if let Some(t) = style["themeColor"].as_str() {
        return Some(format!("theme:{}", t.to_ascii_lowercase()));
    }
    color_hex(&style["rgbColor"]).or_else(|| color_hex(plain))
}

fn style_of(f: &J) -> Style {
    let tf = &f["textFormat"];
    let bg = color_of(&f["backgroundColorStyle"], &f["backgroundColor"]).filter(|c| c != "#FFFFFF");
    let fg = color_of(&tf["foregroundColorStyle"], &tf["foregroundColor"]).filter(|c| c != "#000000");
    let font = tf["fontFamily"].as_str().filter(|f| *f != "Arial").map(Into::into);
    let size = tf["fontSize"].as_u64().map(|s| s as u32).filter(|s| *s != 10);
    let borders = f["borders"].as_object().map(|b| b.values().any(|v| v["style"].as_str().is_some_and(|s| s != "NONE"))).unwrap_or(false);
    Style {
        num_fmt: f["numberFormat"]["pattern"].as_str().map(Into::into),
        num_type: f["numberFormat"]["type"].as_str().map(Into::into),
        bg,
        fg,
        bold: tf["bold"].as_bool().unwrap_or(false),
        italic: tf["italic"].as_bool().unwrap_or(false),
        strike: tf["strikethrough"].as_bool().unwrap_or(false),
        underline: tf["underline"].as_bool().unwrap_or(false),
        borders,
        h_align: f["horizontalAlignment"].as_str().filter(|a| *a != "LEFT").map(Into::into),
        font,
        size,
        wrap: f["wrapStrategy"].as_str() == Some("WRAP"),
    }
}

fn cond_values(cond: &J) -> Vec<String> {
    arr(&cond["values"])
        .iter()
        .map(|v| v["userEnteredValue"].as_str().or(v["relativeDate"].as_str()).unwrap_or("").to_string())
        .collect()
}

fn validation(d: &J) -> Validation {
    let cond = &d["condition"];
    Validation {
        kind: cond["type"].as_str().unwrap_or("?").into(),
        values: cond_values(cond),
        strict: d["strict"].as_bool().unwrap_or(false),
        dropdown: d["showCustomUi"].as_bool().unwrap_or(false),
        help: d["inputMessage"].as_str().map(Into::into),
    }
}

fn style_brief(f: &J) -> String {
    let s = style_of(f);
    let mut out = Vec::new();
    if let Some(b) = s.bg {
        out.push(format!("bg{b}"));
    }
    if let Some(c) = s.fg {
        out.push(format!("fg{c}"));
    }
    if s.bold {
        out.push("bold".into());
    }
    if s.italic {
        out.push("italic".into());
    }
    if s.strike {
        out.push("strike".into());
    }
    if let Some(n) = s.num_fmt {
        out.push(format!("fmt {n}"));
    }
    out.join(" ")
}

fn cond_rule(cf: &J) -> String {
    if let Some(b) = cf.get("booleanRule") {
        let cond = &b["condition"];
        let vals = cond_values(cond);
        format!("{} {} → {}", cond["type"].as_str().unwrap_or("?"), vals.join(", "), style_brief(&b["format"]))
    } else if let Some(g) = cf.get("gradientRule") {
        let pt = |p: &J| {
            if p.is_object() {
                format!("{}{}:{}", p["type"].as_str().unwrap_or(""), p["value"].as_str().map(|v| format!(" {v}")).unwrap_or_default(), color_of(&p["colorStyle"], &p["color"]).unwrap_or_default())
            } else {
                String::new()
            }
        };
        let parts: Vec<String> = [pt(&g["minpoint"]), pt(&g["midpoint"]), pt(&g["maxpoint"])].into_iter().filter(|s| !s.is_empty()).collect();
        format!("color scale {}", parts.join(" → "))
    } else {
        "?".into()
    }
}

fn filter(f: &J, title: Option<String>) -> Filter {
    let mut criteria = Vec::new();
    for spec in arr(&f["filterSpecs"]) {
        let col = spec["columnIndex"].as_u64().unwrap_or(0) as u32;
        let c = &spec["filterCriteria"];
        let mut bits = Vec::new();
        if let Some(h) = c["hiddenValues"].as_array() {
            bits.push(format!("hide {} values", h.len()));
        }
        if c["condition"].is_object() {
            bits.push(format!("{} {}", c["condition"]["type"].as_str().unwrap_or("?"), cond_values(&c["condition"]).join(",")));
        }
        if !bits.is_empty() {
            criteria.push(format!("{}: {}", col_name(col), bits.join("; ")));
        }
    }
    Filter { range: grid_range(&f["range"]), title, criteria }
}

fn chart(c: &J, id_to_name: &HashMap<i64, String>) -> Chart {
    let spec = &c["spec"];
    let kind = spec
        .as_object()
        .and_then(|o| {
            if let Some(b) = o.get("basicChart") {
                return b["chartType"].as_str().map(|s| s.to_ascii_lowercase());
            }
            o.keys().find(|k| k.ends_with("Chart")).cloned()
        })
        .unwrap_or_else(|| "chart".into());
    let mut sources = Vec::new();
    collect_sources(spec, id_to_name, &mut sources);
    sources.dedup();
    Chart { id: c["chartId"].to_string(), title: spec["title"].as_str().unwrap_or_default().into(), kind, sources }
}

fn collect_sources(j: &J, id_to_name: &HashMap<i64, String>, out: &mut Vec<String>) {
    match j {
        J::Object(o) => {
            if let Some(srcs) = o.get("sourceRange").and_then(|s| s["sources"].as_array()) {
                for g in srcs {
                    out.push(grid_range_a1(g, id_to_name));
                }
            }
            for (k, v) in o {
                if k != "sourceRange" {
                    collect_sources(v, id_to_name, out);
                }
            }
        }
        J::Array(a) => a.iter().for_each(|v| collect_sources(v, id_to_name, out)),
        _ => {}
    }
}

fn pivot(p: &J, anchor: (u32, u32), id_to_name: &HashMap<i64, String>) -> Pivot {
    let src = &p["source"];
    let c0 = src["startColumnIndex"].as_u64().unwrap_or(0) as u32;
    let col = |g: &J| {
        g["sourceColumnOffset"].as_u64().map(|o| col_name(c0 + o as u32)).or_else(|| g["dataSourceColumnReference"]["name"].as_str().map(Into::into)).unwrap_or_default()
    };
    let group = |k: &str| arr(&p[k]).iter().map(col).collect::<Vec<_>>();
    Pivot {
        anchor,
        source: if src.is_object() { grid_range_a1(src, id_to_name) } else { format!("dataSource {}", p["dataSourceId"].as_str().unwrap_or("?")) },
        rows: group("rows"),
        cols: group("columns"),
        values: arr(&p["values"])
            .iter()
            .map(|v| match v["formula"].as_str() {
                Some(f) => format!("{} {}", v["name"].as_str().unwrap_or(""), f),
                None => format!("{}({})", v["summarizeFunction"].as_str().unwrap_or("SUM"), col(v)),
            })
            .collect(),
        filters: arr(&p["filterSpecs"]).iter().map(col).collect(),
    }
}
