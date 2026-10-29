//! Macro code: the procedures in each script module and, for VBA, which sheets
//! and ranges each one reads or changes, which procedures it calls, and which
//! event runs it.
//!
//! This reads the source without running it, so it follows what is written
//! out: `Worksheets("Data").Range("A1:B5")`, a sheet or range held in a
//! variable (`Set ws = …`), a `With` block, a sheet passed to another
//! procedure's parameter, a code name (`Sheet1.Range`), a named range. An
//! address built at run time (`Range("A" & i)`, `Cells(i, 2)`) counts as
//! somewhere on that sheet.

use crate::a1::{col_index, parse_rect, quote_sheet, Rect};
use crate::analyze::{sheet_index, Analysis};
use crate::model::Workbook;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Clone)]
pub struct Proc {
    /// Index into `wb.scripts`.
    pub script: usize,
    pub name: String,
    /// "sub" | "function" | "property"
    pub kind: &'static str,
    /// First and last line in the module, 1-based.
    pub lines: (usize, usize),
    pub trigger: Option<Trigger>,
    /// Sorted by (sheet, range).
    pub touches: Vec<Touch>,
    /// Indices into the procedure list, sorted.
    pub calls: Vec<usize>,
    /// Declared `As Worksheet`: a helper that hands back a sheet by name.
    pub returns_sheet: bool,
}

/// An event that runs a procedure without anyone calling it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trigger {
    /// "Open", "Change", "BeforeClose", … (Apps Script: "onEdit", "doGet", …)
    pub event: String,
    /// The sheet whose event it is; None for the workbook.
    pub sheet: Option<usize>,
}

/// The sheet a reference lands on.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum At {
    Sheet(usize),
    /// Named in the code but not in the file: one the macro creates, or one
    /// since deleted or renamed.
    Missing(String),
    /// Whichever sheet is active when it runs.
    Active,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Touch {
    pub at: At,
    /// None: somewhere on the sheet (an address built at run time).
    pub rect: Option<Rect>,
    /// Assigns to it or changes it (`.Value = …`, `.ClearContents`, `.Delete`, …).
    pub write: bool,
}

/// Methods that change the range or sheet they're called on.
const MUTATORS: &[&str] = &[
    "add", "autofill", "autofit", "clear", "clearcomments", "clearcontents", "clearformats", "copypicture", "delete", "filldown",
    "fillright", "insert", "merge", "pastespecial", "removeduplicates", "replace", "sort", "texttocolumns", "unmerge",
];
/// Members that move away from the literal address, so only the sheet is known.
const RELATIVE: &[&str] = &["columns", "currentregion", "end", "entirecolumn", "entirerow", "offset", "resize", "rows", "specialcells", "usedrange"];
/// Statements whose `=` compares rather than assigns.
const NO_ASSIGN: &[&str] = &[
    "call", "case", "debug", "do", "exit", "for", "goto", "loop", "msgbox", "next", "on", "resume", "return", "select", "until", "wend", "while",
];
/// Apps Script simple triggers and web-app entry points.
const JS_TRIGGERS: &[&str] = &["onOpen", "onEdit", "onChange", "onSelectionChange", "onFormSubmit", "onInstall", "doGet", "doPost"];

pub fn analyze(wb: &Workbook, a: &Analysis) -> Vec<Proc> {
    let mut procs = Vec::new();
    let mut bodies: Vec<Vec<Vec<Tok>>> = Vec::new();
    let mut params: Vec<Vec<String>> = Vec::new();
    let code_names: HashMap<String, usize> =
        wb.sheets.iter().enumerate().filter_map(|(i, s)| Some((s.code_name.as_ref()?.to_ascii_lowercase(), i))).collect();
    // String constants stand in for sheet names and addresses
    // (`Const RAW As String = "Raw Data"`); a module's own come first.
    let mut global_consts = HashMap::new();
    let mut module_consts = Vec::new();
    for sc in &wb.scripts {
        let mut mine = HashMap::new();
        if sc.kind == "vba" {
            for (_, st) in statements(&sc.source) {
                if let Some((name, value, public)) = string_const(&st) {
                    if public {
                        global_consts.entry(name.clone()).or_insert_with(|| value.clone());
                    }
                    mine.insert(name, value);
                }
            }
        }
        module_consts.push(mine);
    }
    for (si, sc) in wb.scripts.iter().enumerate() {
        if sc.kind == "vba" {
            let me = code_names.get(&sc.name.to_ascii_lowercase()).copied();
            let mut open: Option<(Proc, Vec<String>, Vec<Vec<Tok>>)> = None;
            let mut last = 0;
            for (line, mut st) in statements(&sc.source) {
                last = line;
                for t in &mut st {
                    if let Tok::Id(w) = t {
                        let lw = w.to_ascii_lowercase();
                        if let Some(v) = module_consts[si].get(&lw).or_else(|| global_consts.get(&lw)) {
                            *t = Tok::Str(v.clone());
                        }
                    }
                }
                if let Some((kind, name, ps, returns_sheet)) = proc_header(&st) {
                    let trigger = vba_trigger(&name, me);
                    let p = Proc { script: si, name, kind, lines: (line, line), trigger, touches: Vec::new(), calls: Vec::new(), returns_sheet };
                    open = Some((p, ps, Vec::new()));
                } else if id_is(st.first(), "end") && ["sub", "function", "property"].iter().any(|k| id_is(st.get(1), k)) {
                    if let Some((mut p, ps, body)) = open.take() {
                        p.lines.1 = line;
                        procs.push(p);
                        params.push(ps);
                        bodies.push(body);
                    }
                } else if let Some((_, _, body)) = &mut open {
                    body.push(st);
                }
            }
            // A module cut off before `End Sub` still has the procedure.
            if let Some((mut p, ps, body)) = open {
                p.lines.1 = last.max(p.lines.0);
                procs.push(p);
                params.push(ps);
                bodies.push(body);
            }
        } else {
            let total = sc.source.lines().count();
            let start = procs.len();
            for (n, line) in sc.source.lines().enumerate() {
                if let Some(name) = js_function(line) {
                    let trigger = JS_TRIGGERS.contains(&name.as_str()).then(|| Trigger { event: name.clone(), sheet: None });
                    procs.push(Proc { script: si, name, kind: "function", lines: (n + 1, total), trigger, touches: Vec::new(), calls: Vec::new(), returns_sheet: false });
                    params.push(Vec::new());
                    bodies.push(Vec::new());
                }
            }
            // Each function runs until the next one starts.
            let starts: Vec<usize> = procs[start..].iter().skip(1).map(|p| p.lines.0).collect();
            for (p, next) in procs[start..].iter_mut().zip(starts) {
                p.lines.1 = next - 1;
            }
        }
    }

    let mut by_name: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, p) in procs.iter().enumerate() {
        by_name.entry(p.name.to_ascii_lowercase()).or_default().push(i);
    }
    // Sheets passed as arguments bind the callee's parameters, which can pass
    // them on again: repeat until no binding grows.
    let mut bindings: Vec<HashMap<String, Binding>> = vec![HashMap::new(); procs.len()];
    for _ in 0..10 {
        let mut next = bindings.clone();
        for (pi, body) in bodies.iter().enumerate() {
            if wb.scripts[procs[pi].script].kind != "vba" {
                continue;
            }
            let me = code_names.get(&wb.scripts[procs[pi].script].name.to_ascii_lowercase()).copied();
            let mut sc = Scope { wb, a, code_names: &code_names, by_name: &by_name, procs: &procs, me, this: pi, vars: bindings[pi].clone(), with: Vec::new() };
            let mut out = Out::default();
            for st in body {
                sc.statement(st, &mut out);
            }
            let mut touches: BTreeMap<(At, Option<Rect>), bool> = BTreeMap::new();
            for t in out.touches {
                *touches.entry((t.at, t.rect)).or_default() |= t.write;
            }
            procs[pi].touches = touches.into_iter().map(|((at, rect), write)| Touch { at, rect, write }).collect();
            procs[pi].calls = out.calls.iter().map(|(c, _)| *c).collect::<BTreeSet<_>>().into_iter().collect();
            for (c, args) in out.calls {
                for (k, b) in args.into_iter().enumerate() {
                    if let (Some(b), Some(p)) = (b, params[c].get(k)) {
                        next[c].entry(p.clone()).or_default().merge(&b);
                    }
                }
            }
        }
        if next == bindings {
            break;
        }
        bindings = next;
    }
    procs
}

/// Formula cells that call this procedure (a VBA function used as a worksheet function).
pub fn formula_cells(a: &Analysis, p: &Proc) -> u64 {
    if p.kind != "function" {
        return 0;
    }
    a.custom_funcs.iter().filter(|(f, _)| f.eq_ignore_ascii_case(&p.name)).map(|(_, n)| n).sum()
}

fn sheet_label(wb: &Workbook, at: &At) -> String {
    match at {
        At::Sheet(s) => quote_sheet(&wb.sheets[*s].name),
        At::Missing(n) => quote_sheet(n),
        At::Active => "ActiveSheet".into(),
    }
}

/// `Sheet!A1`, `Sheet`, `Out!A1 (not in file)`, `ActiveSheet!A1`.
pub fn place(wb: &Workbook, t: &Touch) -> String {
    let mut s = sheet_label(wb, &t.at);
    if let Some(r) = t.rect {
        s += &format!("!{r}");
    }
    if matches!(t.at, At::Missing(_)) {
        s += " (not in file)";
    }
    s
}

/// One line on what a procedure does: its trigger, formula use, the ranges it
/// writes and uses, the procedures it calls. `max` caps each list. None when
/// there's nothing to say beyond its name.
/// Whether a shape's `on_click` (`RollUp` or `Module1.RollUp`) names this procedure.
pub fn clicks(on_click: &str, wb: &Workbook, p: &Proc) -> bool {
    match on_click.rsplit_once('.') {
        Some((m, n)) => n.eq_ignore_ascii_case(&p.name) && m.eq_ignore_ascii_case(&wb.scripts[p.script].name),
        None => on_click.eq_ignore_ascii_case(&p.name),
    }
}

pub fn describe(wb: &Workbook, a: &Analysis, p: &Proc, max: usize) -> Option<String> {
    let list = |v: Vec<String>, sep: &str| {
        let more = v.len().saturating_sub(max);
        let mut s = v.into_iter().take(max).collect::<Vec<_>>().join(sep);
        if more > 0 {
            s += &format!(" +{more} more");
        }
        s
    };
    let mut parts = Vec::new();
    if let Some(t) = &p.trigger {
        match t.sheet {
            Some(s) => parts.push(format!("runs on {} {}", quote_sheet(&wb.sheets[s].name), t.event)),
            None if wb.scripts[p.script].kind == "vba" => parts.push(format!("runs on workbook {}", t.event)),
            None => parts.push(format!("trigger {}", t.event)),
        }
    }
    for sh in &wb.sheets {
        for x in sh.shapes.iter().filter(|x| x.on_click.as_deref().is_some_and(|m| clicks(m, wb, p))) {
            let label = x.text.as_deref().or(Some(x.name.as_str()).filter(|n| !n.is_empty())).map(|t| format!(" \"{}\"", crate::report::trunc(t, 40)));
            parts.push(format!("run by {}{} on {}", x.kind, label.unwrap_or_default(), quote_sheet(&sh.name)));
        }
    }
    let n = formula_cells(a, p);
    if n > 0 {
        parts.push(format!("used in {n} formula cell{}", if n == 1 { "" } else { "s" }));
    }
    // Per sheet: `Sheet!A1,B2:C5` (ranges inside another listed one dropped),
    // with `,…` when it also reaches cells whose address is built at run time.
    for (write, verb) in [(true, "writes"), (false, "uses")] {
        let mut by_sheet: BTreeMap<&At, (bool, Vec<Rect>)> = BTreeMap::new();
        for t in p.touches.iter().filter(|t| t.write == write) {
            let e = by_sheet.entry(&t.at).or_default();
            match t.rect {
                Some(r) => e.1.push(r),
                None => e.0 = true,
            }
        }
        let inside = |r: &Rect, o: &Rect| o != r && o.r0 <= r.r0 && r.r1 <= o.r1 && o.c0 <= r.c0 && r.c1 <= o.c1;
        let sheets: Vec<String> = by_sheet
            .into_iter()
            .map(|(at, (whole, rects))| {
                let rects: Vec<String> = rects.iter().filter(|r| !rects.iter().any(|o| inside(r, o))).map(|r| r.a1()).collect();
                let mut s = sheet_label(wb, at);
                if !rects.is_empty() {
                    let more = rects.len().saturating_sub(max);
                    s += &format!("!{}", rects.into_iter().take(max).collect::<Vec<_>>().join(","));
                    if more > 0 {
                        s += &format!(",+{more}");
                    }
                    if whole {
                        s += ",…";
                    }
                }
                if matches!(at, At::Missing(_)) {
                    s += " (not in file)";
                }
                s
            })
            .collect();
        if !sheets.is_empty() {
            parts.push(format!("{verb} {}", list(sheets, "; ")));
        }
    }
    if !p.calls.is_empty() {
        parts.push(format!("calls {}", list(p.calls.iter().map(|&c| a.macros[c].name.clone()).collect(), ", ")));
    }
    (!parts.is_empty()).then(|| parts.join(" · "))
}

fn vba_trigger(name: &str, me: Option<usize>) -> Option<Trigger> {
    let lower = name.to_ascii_lowercase();
    if let Some(ev) = lower.strip_prefix("workbook_") {
        return Some(Trigger { event: name[name.len() - ev.len()..].to_string(), sheet: None });
    }
    if let Some(ev) = lower.strip_prefix("worksheet_")
        && me.is_some()
    {
        return Some(Trigger { event: name[name.len() - ev.len()..].to_string(), sheet: me });
    }
    match lower.as_str() {
        "auto_open" => Some(Trigger { event: "Open".into(), sheet: None }),
        "auto_close" => Some(Trigger { event: "BeforeClose".into(), sheet: None }),
        _ => None,
    }
}

fn js_function(line: &str) -> Option<String> {
    let t = line.trim_start();
    let rest = ["async function ", "function "].iter().find_map(|k| t.strip_prefix(k))?;
    let name: String = rest.trim_start().chars().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '$').collect();
    (!name.is_empty()).then_some(name)
}

// ---------------------------------------------------------------- tokens

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Id(String),
    Str(String),
    Num(f64),
    Op(String),
}

fn id_is(t: Option<&Tok>, w: &str) -> bool {
    matches!(t, Some(Tok::Id(s)) if s.eq_ignore_ascii_case(w))
}

fn op_is(t: Option<&Tok>, o: &str) -> bool {
    matches!(t, Some(Tok::Op(s)) if s == o)
}

/// One line of VBA, up to its comment.
fn tokenize(line: &str) -> Vec<Tok> {
    let cs: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '\'' {
            break;
        } else if c == '"' {
            let mut s = String::new();
            i += 1;
            while i < cs.len() {
                if cs[i] == '"' {
                    if cs.get(i + 1) == Some(&'"') {
                        s.push('"');
                        i += 2;
                        continue;
                    }
                    i += 1;
                    break;
                }
                s.push(cs[i]);
                i += 1;
            }
            out.push(Tok::Str(s));
        } else if c.is_ascii_digit() {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.') {
                i += 1;
            }
            out.push(Tok::Num(cs[st..i].iter().collect::<String>().parse().unwrap_or(f64::NAN)));
        } else if c.is_alphabetic() || c == '_' {
            let st = i;
            while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            let id: String = cs[st..i].iter().collect();
            if cs.get(i) == Some(&'$') {
                i += 1; // Left$(…)
            }
            if id.eq_ignore_ascii_case("rem") && out.is_empty() {
                break;
            }
            out.push(Tok::Id(id));
        } else {
            let two: String = cs[i..(i + 2).min(cs.len())].iter().collect();
            if [":=", "<=", ">=", "<>"].contains(&two.as_str()) {
                out.push(Tok::Op(two));
                i += 2;
            } else {
                out.push(Tok::Op(c.to_string()));
                i += 1;
            }
        }
    }
    out
}

/// A module's statements with the line each starts on: continuation lines
/// (` _`) joined, `:`-separated statements split.
fn statements(src: &str) -> Vec<(usize, Vec<Tok>)> {
    let mut out = Vec::new();
    let mut cur: Vec<Tok> = Vec::new();
    let mut start = 0;
    for (n, line) in src.lines().enumerate() {
        if cur.is_empty() {
            start = n + 1;
        }
        let mut toks = tokenize(line);
        let cont = matches!(toks.last(), Some(Tok::Id(s)) if s == "_");
        if cont {
            toks.pop();
        }
        cur.extend(toks);
        if !cont {
            for st in std::mem::take(&mut cur).split(|t| op_is(Some(t), ":")) {
                if !st.is_empty() {
                    out.push((start, st.to_vec()));
                }
            }
        }
    }
    if !cur.is_empty() {
        out.push((start, cur));
    }
    out
}

/// `[Public|Private] Const Name [As String] = "text"`: (lowercased name, text, public).
fn string_const(t: &[Tok]) -> Option<(String, String, bool)> {
    let public = id_is(t.first(), "public") || id_is(t.first(), "global");
    let i = usize::from(public || id_is(t.first(), "private"));
    if !id_is(t.get(i), "const") {
        return None;
    }
    let Some(Tok::Id(name)) = t.get(i + 1) else { return None };
    let eq = t.iter().position(|x| op_is(Some(x), "="))?;
    match &t[eq + 1..] {
        [Tok::Str(v)] => Some((name.to_ascii_lowercase(), v.clone(), public)),
        _ => None,
    }
}

/// `[Public|Private|Friend|Static] Sub|Function|Property Get|Let|Set Name(params)`.
fn proc_header(t: &[Tok]) -> Option<(&'static str, String, Vec<String>, bool)> {
    let mut i = 0;
    while ["public", "private", "friend", "static"].iter().any(|k| id_is(t.get(i), k)) {
        i += 1;
    }
    let kind = if id_is(t.get(i), "sub") {
        "sub"
    } else if id_is(t.get(i), "function") {
        "function"
    } else if id_is(t.get(i), "property") {
        i += 1;
        "property"
    } else {
        return None;
    };
    let Some(Tok::Id(name)) = t.get(i + 1) else { return None };
    let mut params = Vec::new();
    let mut close = i + 1;
    if op_is(t.get(i + 2), "(") {
        close = close_paren(t, i + 2);
        for arg in split_args(&t[i + 3..close]) {
            let mut j = 0;
            while ["optional", "byval", "byref", "paramarray"].iter().any(|k| id_is(arg.get(j), k)) {
                j += 1;
            }
            if let Some(Tok::Id(p)) = arg.get(j) {
                params.push(p.to_ascii_lowercase());
            }
        }
    }
    let sheet = id_is(t.get(close + 1), "as") && id_is(t.get(close + 2), "worksheet");
    Some((kind, name.clone(), params, sheet))
}

/// Index of the `)` closing the `(` at `open`, or the end.
fn close_paren(t: &[Tok], open: usize) -> usize {
    let mut depth = 0;
    for (i, tok) in t.iter().enumerate().skip(open) {
        if op_is(Some(tok), "(") {
            depth += 1;
        } else if op_is(Some(tok), ")") {
            depth -= 1;
            if depth == 0 {
                return i;
            }
        }
    }
    t.len()
}

fn split_args(t: &[Tok]) -> Vec<&[Tok]> {
    if t.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let (mut depth, mut start) = (0i32, 0);
    for (i, tok) in t.iter().enumerate() {
        match tok {
            Tok::Op(o) if o == "(" => depth += 1,
            Tok::Op(o) if o == ")" => depth -= 1,
            Tok::Op(o) if o == "," && depth == 0 => {
                out.push(&t[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&t[start..]);
    out
}

// ---------------------------------------------------------------- resolution

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Place {
    at: At,
    rect: Option<Rect>,
}

/// What a variable, parameter or expression refers to.
#[derive(Debug, Clone, Default, PartialEq)]
struct Binding {
    places: BTreeSet<Place>,
    /// A range rather than a whole sheet, so using it bare reads or writes cells.
    range: bool,
}

impl Binding {
    fn sheet(at: At) -> Binding {
        Binding { places: [Place { at, rect: None }].into(), range: false }
    }
    fn merge(&mut self, o: &Binding) {
        self.places.extend(o.places.iter().cloned());
        self.range |= o.range;
    }
}

struct Hit {
    b: Binding,
    write: bool,
    /// Reaches cells or the sheet itself, not just names a sheet to pass on.
    counts: bool,
    end: usize,
}

#[derive(Default)]
struct Out {
    touches: Vec<Touch>,
    /// (callee, what each argument refers to)
    calls: Vec<(usize, Vec<Option<Binding>>)>,
}

struct Scope<'a> {
    wb: &'a Workbook,
    a: &'a Analysis,
    code_names: &'a HashMap<String, usize>,
    by_name: &'a HashMap<String, Vec<usize>>,
    procs: &'a [Proc],
    /// The sheet whose module this is: `Me`, and where a bare `Range` points.
    me: Option<usize>,
    this: usize,
    vars: HashMap<String, Binding>,
    with: Vec<Binding>,
}

impl Scope<'_> {
    fn statement(&mut self, t: &[Tok], out: &mut Out) {
        let head = match t.first() {
            Some(Tok::Id(w)) => w.to_ascii_lowercase(),
            _ => String::new(),
        };
        match head.as_str() {
            // The target of `With` / `Set` is reached through what follows,
            // so only calls in it are recorded here.
            "with" => {
                let b = self.whole(&t[1..]);
                self.scan(&t[1..], None, out, b.is_none());
                self.with.push(b.unwrap_or_default());
            }
            "end" => {
                if id_is(t.get(1), "with") {
                    self.with.pop();
                }
            }
            "set" if matches!(t.get(1), Some(Tok::Id(_))) && op_is(t.get(2), "=") => {
                let Some(Tok::Id(v)) = t.get(1) else { return };
                let b = self.whole(&t[3..]);
                self.scan(&t[3..], None, out, b.is_none());
                match b {
                    Some(b) => self.vars.insert(v.to_ascii_lowercase(), b),
                    None => self.vars.remove(&v.to_ascii_lowercase()),
                };
            }
            "dim" | "redim" | "const" | "attribute" | "option" | "declare" | "public" | "private" | "global" => {}
            "if" | "elseif" => {
                let then = t.iter().position(|x| id_is(Some(x), "then")).unwrap_or(t.len());
                self.scan(&t[1..then], None, out, true);
                if then + 1 < t.len() {
                    self.statement(&t[then + 1..], out);
                }
            }
            "else" => self.statement(&t[1..], out),
            h if NO_ASSIGN.contains(&h) => self.scan(t, None, out, true),
            _ => {
                // A single-line If's `… Else …` tail is its own statement.
                if let Some(e) = t.iter().position(|x| id_is(Some(x), "else")) {
                    self.statement(&t[..e], out);
                    self.statement(&t[e..], out);
                    return;
                }
                // `ws.Name = "Out"`: ws now holds the sheet called Out.
                if let [Tok::Id(v), Tok::Op(d), Tok::Id(n), Tok::Op(e), Tok::Str(name)] = t
                    && d == "."
                    && n.eq_ignore_ascii_case("name")
                    && e == "="
                {
                    let at = sheet_index(self.wb, name).map_or_else(|| At::Missing(name.clone()), At::Sheet);
                    self.vars.insert(v.to_ascii_lowercase(), Binding::sheet(at));
                }
                let mut depth = 0;
                let eq = t.iter().position(|x| {
                    match x {
                        Tok::Op(o) if o == "(" => depth += 1,
                        Tok::Op(o) if o == ")" => depth -= 1,
                        _ => {}
                    }
                    depth == 0 && op_is(Some(x), "=")
                });
                self.scan(t, eq, out, true);
            }
        }
    }

    /// Record the calls in `t` and, with `touches`, every reference; those
    /// before `eq` are assigned to.
    fn scan(&self, t: &[Tok], eq: Option<usize>, out: &mut Out, touches: bool) {
        let mut i = 0;
        while i < t.len() {
            if let Tok::Id(w) = &t[i]
                && !op_is(i.checked_sub(1).and_then(|p| t.get(p)), ".")
                && let Some(callees) = self.callees(w)
            {
                let args = if op_is(t.get(i + 1), "(") {
                    split_args(&t[i + 2..close_paren(t, i + 1).min(t.len())])
                } else if i == 0 || id_is(t.get(i - 1), "call") {
                    split_args(&t[i + 1..])
                } else {
                    Vec::new()
                };
                let bound: Vec<Option<Binding>> = args.iter().map(|a| self.whole(a)).collect();
                for c in callees {
                    out.calls.push((c, bound.clone()));
                }
            }
            match self.resolve(t, i) {
                Some(h) => {
                    if touches && h.counts {
                        let write = h.write || eq.is_some_and(|e| i < e);
                        out.touches.extend(h.b.places.iter().map(|p| Touch { at: p.at.clone(), rect: p.rect, write }));
                    }
                    i = h.end.max(i + 1);
                }
                None => i += 1,
            }
        }
    }

    /// Procedures `w` may call: the one in this module, else any of that name.
    fn callees(&self, w: &str) -> Option<Vec<usize>> {
        let all = self.by_name.get(&w.to_ascii_lowercase())?;
        let mine = self.procs[self.this].script;
        let same: Vec<usize> = all.iter().copied().filter(|&c| self.procs[c].script == mine).collect();
        let v: Vec<usize> = if same.is_empty() { all.clone() } else { same };
        let v: Vec<usize> = v.into_iter().filter(|&c| c != self.this).collect();
        (!v.is_empty()).then_some(v)
    }

    /// What `t` as a whole refers to, allowing a workbook prefix
    /// (`ThisWorkbook.Worksheets("Data")`).
    fn whole(&self, t: &[Tok]) -> Option<Binding> {
        for k in 0..t.len() {
            if let Some(h) = self.resolve(t, k) {
                return (h.end == t.len()).then_some(h.b);
            }
            if !matches!(t[k], Tok::Id(_)) && !op_is(Some(&t[k]), ".") {
                return None;
            }
        }
        None
    }

    /// A reference starting at `t[i]`: its base (sheet expression, variable,
    /// code name, `With` target, bare `Range`) and the member chain after it.
    fn resolve(&self, t: &[Tok], i: usize) -> Option<Hit> {
        let prev_dot = i > 0 && op_is(t.get(i - 1), ".");
        let (mut b, mut j, mut counts, mut write) = match &t[i] {
            Tok::Op(o) if o == "." => {
                let leading = i == 0 || matches!(&t[i - 1], Tok::Op(p) if p != ")");
                let top = self.with.last().filter(|b| leading && !b.places.is_empty())?;
                (top.clone(), i, false, false)
            }
            Tok::Id(w) => {
                let lw = w.to_ascii_lowercase();
                if lw == "worksheets" || lw == "sheets" {
                    let at = self.sheet_arg(t, i + 1, true, true)?;
                    (Binding::sheet(at), i + 4, true, false)
                } else if prev_dot {
                    return None;
                } else if lw == "range" || lw == "cells" {
                    let mut b = Binding::sheet(self.me_at());
                    let mut write = false;
                    let j = self.member(&mut b, &mut write, t, i);
                    (b, j, true, write)
                } else if lw == "me" {
                    (Binding::sheet(At::Sheet(self.me?)), i + 1, false, false)
                } else if lw == "activesheet" {
                    (Binding::sheet(At::Active), i + 1, false, false)
                } else if let Some(v) = self.vars.get(&lw) {
                    (v.clone(), i + 1, v.range, false)
                } else if let Some(&s) = self.code_names.get(&lw) {
                    (Binding::sheet(At::Sheet(s)), i + 1, false, false)
                } else {
                    // A helper given a sheet's name: SheetByName("Data")
                    let creates = self.callees(w).is_some_and(|c| c.iter().any(|&c| self.procs[c].returns_sheet));
                    (Binding::sheet(self.sheet_arg(t, i + 1, false, creates)?), i + 4, true, false)
                }
            }
            _ => return None,
        };
        while op_is(t.get(j), ".") && matches!(t.get(j + 1), Some(Tok::Id(_))) {
            counts = true;
            j = self.member(&mut b, &mut write, t, j + 1);
        }
        Some(Hit { b, write, counts, end: j })
    }

    /// Apply the member at `t[m]` (with its arguments) to `b`; returns the index after it.
    fn member(&self, b: &mut Binding, write: &mut bool, t: &[Tok], m: usize) -> usize {
        let Some(Tok::Id(name)) = t.get(m) else { return m + 1 };
        let name = name.to_ascii_lowercase();
        let (args, next) = if op_is(t.get(m + 1), "(") {
            let c = close_paren(t, m + 1);
            (Some(&t[m + 2..c.min(t.len())]), c + 1)
        } else {
            (None, m + 1)
        };
        if (name == "range" || name == "cells") && !b.range {
            let (sheet, rect) = self.range_args(&name, args);
            b.places = b.places.iter().map(|p| Place { at: sheet.map_or_else(|| p.at.clone(), At::Sheet), rect }).collect();
            b.range = true;
        } else if RELATIVE.contains(&name.as_str()) || ((name == "range" || name == "cells") && args.is_some()) {
            b.places = b.places.iter().map(|p| Place { at: p.at.clone(), rect: None }).collect();
            b.range = true;
        } else if MUTATORS.contains(&name.as_str()) {
            *write = true;
        }
        next
    }

    fn me_at(&self) -> At {
        self.me.map_or(At::Active, At::Sheet)
    }

    /// `("Name")` naming a sheet of this workbook (or, when `sure` it names a
    /// sheet, any name), or `(3)` with `sure`.
    fn sheet_arg(&self, t: &[Tok], k: usize, numeric: bool, sure: bool) -> Option<At> {
        if !op_is(t.get(k), "(") || !op_is(t.get(k + 2), ")") {
            return None;
        }
        match t.get(k + 1)? {
            Tok::Str(s) => match sheet_index(self.wb, s) {
                Some(i) => Some(At::Sheet(i)),
                None => sure.then(|| At::Missing(s.clone())),
            },
            Tok::Num(n) if numeric && *n >= 1.0 && (*n as usize) <= self.wb.sheets.len() => Some(At::Sheet(*n as usize - 1)),
            _ => None,
        }
    }

    /// The sheet (from a named range) and address of `Range(…)` / `Cells(…)`
    /// arguments, when they are written out.
    fn range_args(&self, kind: &str, args: Option<&[Tok]>) -> (Option<usize>, Option<Rect>) {
        let Some(args) = args else { return (None, None) };
        let parts = split_args(args);
        if kind == "cells" {
            return (None, cells_rect(&parts));
        }
        let (mut sheet, mut acc): (Option<usize>, Option<Rect>) = (None, None);
        for p in &parts {
            let r = match p {
                [Tok::Str(s)] => match parse_rect(s) {
                    Some(r) => r,
                    None => {
                        let Some((s, r)) = self.a.name_range(None, s) else { return (None, None) };
                        sheet = s.or(sheet);
                        r
                    }
                },
                // Cells(1, 2), or .Cells(1, 2) inside a With
                [Tok::Id(c), Tok::Op(o), rest @ ..] | [Tok::Op(_), Tok::Id(c), Tok::Op(o), rest @ ..]
                    if c.eq_ignore_ascii_case("cells") && o == "(" && !matches!(&p[0], Tok::Op(x) if x != ".") =>
                {
                    match cells_rect(&split_args(&rest[..rest.len().saturating_sub(1)])) {
                        Some(r) => r,
                        None => return (None, None),
                    }
                }
                _ => return (None, None),
            };
            acc = Some(acc.map_or(r, |a| a.union(&r)));
        }
        (sheet, acc)
    }
}

/// `Cells(2, 3)` / `Cells(2, "C")` with literal arguments.
fn cells_rect(parts: &[&[Tok]]) -> Option<Rect> {
    let [[Tok::Num(r)], [col]] = parts else { return None };
    let c = match col {
        Tok::Num(c) if *c >= 1.0 => *c as u32 - 1,
        Tok::Str(s) => col_index(s)?,
        _ => return None,
    };
    (*r >= 1.0).then(|| Rect::cell(*r as u32 - 1, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statements_join_continuations_and_drop_comments() {
        let st = statements("x = \"it's \"\"quoted\"\"\" ' comment\nIf a Then _\n  b = 1: c = 2\nRem gone");
        assert_eq!(st.len(), 3);
        assert_eq!(st[0].1[2], Tok::Str("it's \"quoted\"".into()));
        assert_eq!((st[1].0, st[1].1.len()), (2, 6));
        assert_eq!((st[2].0, st[2].1.len()), (2, 3));
    }

    #[test]
    fn string_consts() {
        let c = |src: &str| string_const(&statements(src)[0].1);
        assert_eq!(c("Private Const RAW As String = \"Raw Data\""), Some(("raw".into(), "Raw Data".into(), false)));
        assert_eq!(c("Public Const OUT = \"Out\""), Some(("out".into(), "Out".into(), true)));
        assert_eq!(c("Const N As Long = 252"), None);
        assert_eq!(c("Const A = \"x\" & \"y\""), None);
    }

    #[test]
    fn headers() {
        let (kind, name, ps, _) = proc_header(&tokenize("Private Sub Totals(ByVal ws As Worksheet, Optional n As Long = 2)")).unwrap();
        assert_eq!((kind, name.as_str(), ps), ("sub", "Totals", vec!["ws".to_string(), "n".to_string()]));
        assert!(proc_header(&tokenize("Private Function FreshSheet(nm As String) As Worksheet")).unwrap().3);
        assert_eq!(proc_header(&tokenize("Public Property Get Rate() As Double")).unwrap().1, "Rate");
        assert!(proc_header(&tokenize("Private Declare PtrSafe Function GetTickCount Lib \"kernel32\" () As Long")).is_none());
        assert_eq!(js_function("  async function onEdit(e) {").as_deref(), Some("onEdit"));
    }
}
