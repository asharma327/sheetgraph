//! Formula lexing: just enough to find references, function calls and names,
//! and to produce a position-independent (R1C1) key so copied formulas group.

use crate::a1::{parse_pt, pts_rect, Pt, Rect, OPEN};

#[derive(Debug, Clone)]
pub struct RefTok {
    pub span: (usize, usize),
    /// Sheet as written (unquoted). None = same sheet.
    pub sheet: Option<String>,
    pub a: Pt,
    pub b: Option<Pt>,
}

#[derive(Debug, Clone)]
pub struct FuncTok {
    /// Upper-cased, `_xlfn.` prefixes stripped.
    pub name: String,
    /// Byte index of the opening paren.
    pub open: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Lexed {
    pub refs: Vec<RefTok>,
    pub funcs: Vec<FuncTok>,
    /// Bare identifiers that are not functions or refs: named ranges, LET/LAMBDA
    /// params, Excel table names.
    pub names: Vec<String>,
    /// Other workbooks referenced as `'[Budget.xlsx]Sheet'!A1` (Excel links).
    pub books: Vec<String>,
}

fn is_ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'$' || b >= 0x80
}
fn is_refch(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'$'
}

pub fn lex(f: &str) -> Lexed {
    let b = f.as_bytes();
    let mut out = Lexed::default();
    let mut i = if b.first() == Some(&b'=') { 1 } else { 0 };
    while i < b.len() {
        let ch = b[i];
        match ch {
            b'"' => {
                i += 1;
                while i < b.len() {
                    if b[i] == b'"' {
                        if i + 1 < b.len() && b[i + 1] == b'"' {
                            i += 2;
                            continue;
                        }
                        break;
                    }
                    i += 1;
                }
                i += 1;
            }
            b'\'' => {
                // quoted sheet name: '...'!ref
                let start = i;
                i += 1;
                let mut name = Vec::new();
                while i < b.len() {
                    if b[i] == b'\'' {
                        if i + 1 < b.len() && b[i + 1] == b'\'' {
                            name.push(b'\'');
                            i += 2;
                            continue;
                        }
                        break;
                    }
                    name.push(b[i]);
                    i += 1;
                }
                i += 1;
                if i < b.len() && b[i] == b'!' {
                    let sheet = String::from_utf8_lossy(&name).into_owned();
                    if let Some((book, _)) = sheet.strip_prefix('[').and_then(|s| s.split_once(']'))
                        && !out.books.iter().any(|x| x == book) {
                            out.books.push(book.to_string());
                        }
                    i = lex_ref_after_sheet(f, start, i + 1, Some(sheet), &mut out);
                }
            }
            b'[' => {
                // Excel structured refs / external workbook index: skip balanced.
                let mut depth = 0;
                while i < b.len() {
                    if b[i] == b'[' {
                        depth += 1;
                    } else if b[i] == b']' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    i += 1;
                }
                i += 1;
            }
            b'#' => {
                // error literal like #REF! / #N/A
                i += 1;
                while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'/' || b[i] == b'!' || b[i] == b'?') {
                    i += 1;
                }
            }
            _ if is_ident(ch) => {
                let start = i;
                while i < b.len() && is_ident(b[i]) {
                    i += 1;
                }
                let word = &f[start..i];
                let mut j = i;
                while j < b.len() && b[j] == b' ' {
                    j += 1;
                }
                if j < b.len() && b[j] == b'(' {
                    out.funcs.push(FuncTok { name: norm_func(word), open: j });
                    continue;
                }
                if i < b.len() && b[i] == b'!' {
                    i = lex_ref_after_sheet(f, start, i + 1, Some(word.to_string()), &mut out);
                    continue;
                }
                if i < b.len() && b[i] == b'[' {
                    // Table1[Col]
                    out.names.push(word.to_string());
                    continue;
                }
                if let Some(a) = parse_pt(word) {
                    if i < b.len() && b[i] == b':' {
                        let s2 = i + 1;
                        let mut k = s2;
                        while k < b.len() && is_refch(b[k]) {
                            k += 1;
                        }
                        if let Some(bp) = parse_pt(&f[s2..k])
                            && pts_rect(a, Some(bp)).is_some() {
                                out.refs.push(RefTok { span: (start, k), sheet: None, a, b: Some(bp) });
                                i = k;
                                continue;
                            }
                    }
                    if a.col.is_some() && a.row.is_some() {
                        out.refs.push(RefTok { span: (start, i), sheet: None, a, b: None });
                        continue;
                    }
                }
                let first = word.as_bytes()[0];
                if !first.is_ascii_digit() && first != b'.' {
                    let up = word.to_ascii_uppercase();
                    if up != "TRUE" && up != "FALSE" {
                        out.names.push(word.to_string());
                    }
                }
            }
            _ => i += 1,
        }
    }
    out
}

fn lex_ref_after_sheet(f: &str, start: usize, mut i: usize, sheet: Option<String>, out: &mut Lexed) -> usize {
    let b = f.as_bytes();
    let s1 = i;
    while i < b.len() && is_refch(b[i]) {
        i += 1;
    }
    let Some(a) = parse_pt(&f[s1..i]) else {
        // Sheet-scoped name or something we don't model.
        while i < b.len() && is_ident(b[i]) {
            i += 1;
        }
        return i;
    };
    if i < b.len() && b[i] == b':' {
        let s2 = i + 1;
        let mut k = s2;
        while k < b.len() && is_refch(b[k]) {
            k += 1;
        }
        if let Some(bp) = parse_pt(&f[s2..k])
            && pts_rect(a, Some(bp)).is_some() {
                out.refs.push(RefTok { span: (start, k), sheet, a, b: Some(bp) });
                return k;
            }
    }
    if a.col.is_some() && a.row.is_some() {
        out.refs.push(RefTok { span: (start, i), sheet, a, b: None });
    }
    i
}

fn norm_func(w: &str) -> String {
    let up = w.to_ascii_uppercase();
    for p in ["_XLFN._XLWS.", "_XLFN.", "_XLWS.", "_XLUDF."] {
        if let Some(rest) = up.strip_prefix(p) {
            return rest.to_string();
        }
    }
    up
}

impl RefTok {
    /// Rectangle this reference covers when the formula sits at (hr, hc) and
    /// the reference was written for anchor (ar, ac). Relative parts shift.
    pub fn rect_at(&self, dr: i64, dc: i64) -> Option<Rect> {
        let sh = |p: Pt| Pt {
            col: p.col.map(|c| if p.col_abs { c } else { shift(c, dc) }),
            row: p.row.map(|r| if p.row_abs { r } else { shift(r, dr) }),
            ..p
        };
        pts_rect(sh(self.a), self.b.map(sh))
    }
    pub fn rect(&self) -> Option<Rect> {
        pts_rect(self.a, self.b)
    }
}

fn shift(v: u32, d: i64) -> u32 {
    if v == OPEN {
        return v;
    }
    (v as i64 + d).clamp(0, OPEN as i64 - 1) as u32
}

/// Position-independent formula key: every reference rewritten in R1C1 relative
/// to the host cell. Two cells share a key iff one is a copy of the other.
pub fn r1c1_key(f: &str, lx: &Lexed, hr: u32, hc: u32) -> String {
    let mut out = String::with_capacity(f.len() + 8);
    let mut last = 0;
    for r in &lx.refs {
        out.push_str(&f[last..r.span.0]);
        if let Some(s) = &r.sheet {
            out.push_str(s);
            out.push('!');
        }
        push_pt(&mut out, r.a, hr, hc);
        if let Some(b) = r.b {
            out.push(':');
            push_pt(&mut out, b, hr, hc);
        }
        last = r.span.1;
    }
    out.push_str(&f[last..]);
    out
}

fn push_pt(out: &mut String, p: Pt, hr: u32, hc: u32) {
    use std::fmt::Write;
    if let Some(r) = p.row {
        if p.row_abs {
            let _ = write!(out, "R{}", r + 1);
        } else {
            let _ = write!(out, "R[{}]", r as i64 - hr as i64);
        }
    }
    if let Some(c) = p.col {
        if p.col_abs {
            let _ = write!(out, "C{}", c + 1);
        } else {
            let _ = write!(out, "C[{}]", c as i64 - hc as i64);
        }
    }
}

/// Coarser key for "families" of groups: refs and numbers erased, so e.g. the
/// same SUMIFS pointed at different rows collapses.
pub fn family_key(f: &str, lx: &Lexed) -> String {
    let mut out = String::with_capacity(f.len());
    let mut last = 0;
    for r in &lx.refs {
        out.push_str(&f[last..r.span.0]);
        out.push('@');
        last = r.span.1;
    }
    out.push_str(&f[last..]);
    // erase numeric literals
    let mut s = String::with_capacity(out.len());
    let mut prev_digit = false;
    for ch in out.chars() {
        if ch.is_ascii_digit() || (prev_digit && ch == '.') {
            if !prev_digit {
                s.push('#');
            }
            prev_digit = true;
        } else {
            prev_digit = false;
            s.push(ch);
        }
    }
    s
}

/// Top-level arguments of the call whose '(' is at `open`.
pub fn call_args(f: &str, open: usize) -> Vec<String> {
    let b = f.as_bytes();
    let mut args = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    let mut i = open + 1;
    let mut in_str = false;
    while i < b.len() {
        let ch = f[i..].chars().next().unwrap();
        let w = ch.len_utf8();
        if in_str {
            if ch == '"' {
                in_str = false;
            }
            cur.push(ch);
        } else {
            match ch {
                '"' => {
                    in_str = true;
                    cur.push(ch)
                }
                '(' | '{' => {
                    depth += 1;
                    cur.push(ch)
                }
                ')' | '}' if depth == 0 => {
                    args.push(cur.trim().to_string());
                    return args;
                }
                ')' | '}' => {
                    depth -= 1;
                    cur.push(ch)
                }
                ',' | ';' if depth == 0 => {
                    args.push(cur.trim().to_string());
                    cur.clear();
                }
                _ => cur.push(ch),
            }
        }
        i += w;
    }
    args.push(cur.trim().to_string());
    args
}

pub fn unquote(s: &str) -> Option<String> {
    let s = s.trim();
    if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        Some(s[1..s.len() - 1].replace("\"\"", "\""))
    } else {
        None
    }
}

/// Pull a spreadsheet id out of a URL or return the string if it already is one.
pub fn spreadsheet_id(s: &str) -> String {
    if let Some(i) = s.find("/d/") {
        let rest = &s[i + 3..];
        return rest.split(['/', '?', '#']).next().unwrap_or(rest).to_string();
    }
    s.trim().to_string()
}

pub const EXTERNAL_FUNCS: &[&str] = &[
    "IMPORTRANGE", "IMPORTDATA", "IMPORTHTML", "IMPORTXML", "IMPORTFEED", "GOOGLEFINANCE",
    "GOOGLETRANSLATE", "DETECTLANGUAGE", "IMAGE", "WEBSERVICE", "STOCKHISTORY", "RTD",
    "CUBEVALUE", "CUBEMEMBER", "CUBESET", "AI", "GEMINI",
];
pub const DYNAMIC_FUNCS: &[&str] = &["INDIRECT", "OFFSET"];
pub const VOLATILE_FUNCS: &[&str] = &["NOW", "TODAY", "RAND", "RANDBETWEEN", "RANDARRAY"];
pub const SPILL_FUNCS: &[&str] = &[
    "ARRAYFORMULA", "FILTER", "QUERY", "SORT", "SORTN", "UNIQUE", "SEQUENCE", "IMPORTRANGE",
    "SPLIT", "FLATTEN", "TRANSPOSE", "MMULT", "BYROW", "BYCOL", "MAP", "MAKEARRAY", "SCAN",
    "VSTACK", "HSTACK", "TOCOL", "TOROW", "CHOOSECOLS", "CHOOSEROWS", "WRAPROWS", "WRAPCOLS",
    "IMPORTDATA", "IMPORTHTML", "IMPORTXML", "IMPORTFEED", "RANDARRAY", "GOOGLEFINANCE",
];

/// Built-in function names (Google Sheets ∪ Excel). Anything else called in a
/// formula is a custom function (Apps Script / VBA UDF) or a named function.
pub fn is_builtin(name: &str) -> bool {
    static SET: std::sync::OnceLock<std::collections::HashSet<&'static str>> = std::sync::OnceLock::new();
    SET.get_or_init(|| BUILTINS.iter().copied().collect()).contains(name) || name.starts_with("CUBE")
}

pub static BUILTINS: &[&str] = &[
    "ABS", "ACCRINT", "ACCRINTM", "ACOS", "ACOSH", "ACOT", "ACOTH", "ADD", "ADDRESS", "AGGREGATE", "AI", "AMORLINC", "AND", "ARABIC", "ARRAYFORMULA", "ARRAYTOTEXT", "ASC", "ASIN", "ASINH", "ATAN", "ATAN2", "ATANH", "AVEDEV", "AVERAGE", "AVERAGE.WEIGHTED", "AVERAGEA", "AVERAGEIF", "AVERAGEIFS",
    "BAHTTEXT", "BASE", "BETA.DIST", "BETA.INV", "BETADIST", "BETAINV", "BIN2DEC", "BIN2HEX", "BIN2OCT", "BINOM.DIST", "BINOM.INV", "BINOMDIST", "BITAND", "BITLSHIFT", "BITOR", "BITRSHIFT", "BITXOR", "BYCOL", "BYROW",
    "CEILING", "CEILING.MATH", "CEILING.PRECISE", "CELL", "CHAR", "CHIDIST", "CHIINV", "CHISQ.DIST", "CHISQ.DIST.RT", "CHISQ.INV", "CHISQ.INV.RT", "CHISQ.TEST", "CHITEST", "CHOOSE", "CHOOSECOLS", "CHOOSEROWS", "CLEAN", "CODE", "COLUMN", "COLUMNS", "COMBIN", "COMBINA", "COMPLEX", "CONCAT", "CONCATENATE", "CONFIDENCE", "CONFIDENCE.NORM", "CONFIDENCE.T", "CONVERT", "CORREL", "COS", "COSH", "COT", "COTH", "COUNT", "COUNTA", "COUNTBLANK", "COUNTIF", "COUNTIFS", "COUNTUNIQUE", "COUNTUNIQUEIFS", "COUPDAYBS", "COUPDAYS", "COUPDAYSNC", "COUPNCD", "COUPNUM", "COUPPCD", "COVAR", "COVARIANCE.P", "COVARIANCE.S", "CRITBINOM", "CSC", "CSCH", "CUMIPMT", "CUMPRINC",
    "DATE", "DATEDIF", "DATEVALUE", "DAVERAGE", "DAY", "DAYS", "DAYS360", "DB", "DCOUNT", "DCOUNTA", "DDB", "DEC2BIN", "DEC2HEX", "DEC2OCT", "DECIMAL", "DEGREES", "DELTA", "DETECTLANGUAGE", "DEVSQ", "DGET", "DISC", "DIVIDE", "DMAX", "DMIN", "DOLLAR", "DOLLARDE", "DOLLARFR", "DPRODUCT", "DROP", "DSTDEV", "DSTDEVP", "DSUM", "DURATION", "DVAR", "DVARP",
    "EDATE", "EFFECT", "ENCODEURL", "EOMONTH", "EPOCHTODATE", "EQ", "ERF", "ERF.PRECISE", "ERFC", "ERFC.PRECISE", "ERROR.TYPE", "EVEN", "EXACT", "EXP", "EXPAND", "EXPON.DIST", "EXPONDIST",
    "F.DIST", "F.DIST.RT", "F.INV", "F.INV.RT", "F.TEST", "FACT", "FACTDOUBLE", "FALSE", "FDIST", "FILTER", "FILTERXML", "FIND", "FINDB", "FINV", "FISHER", "FISHERINV", "FIXED", "FLATTEN", "FLOOR", "FLOOR.MATH", "FLOOR.PRECISE", "FORECAST", "FORECAST.LINEAR", "FORMULATEXT", "FREQUENCY", "FTEST", "FV", "FVSCHEDULE",
    "GAMMA", "GAMMA.DIST", "GAMMA.INV", "GAMMADIST", "GAMMAINV", "GAMMALN", "GAMMALN.PRECISE", "GAUSS", "GCD", "GEOMEAN", "GESTEP", "GETPIVOTDATA", "GOOGLEFINANCE", "GOOGLETRANSLATE", "GROUPBY", "GROWTH", "GT", "GTE",
    "HARMEAN", "HEX2BIN", "HEX2DEC", "HEX2OCT", "HLOOKUP", "HOUR", "HSTACK", "HYPERLINK", "HYPGEOM.DIST", "HYPGEOMDIST",
    "IF", "IFERROR", "IFNA", "IFS", "IMABS", "IMAGE", "IMAGINARY", "IMARGUMENT", "IMCONJUGATE", "IMCOS", "IMCOSH", "IMCOT", "IMCOTH", "IMCSC", "IMCSCH", "IMDIV", "IMEXP", "IMLN", "IMLOG", "IMLOG10", "IMLOG2", "IMPORTDATA", "IMPORTFEED", "IMPORTHTML", "IMPORTRANGE", "IMPORTXML", "IMPOWER", "IMPRODUCT", "IMREAL", "IMSEC", "IMSECH", "IMSIN", "IMSINH", "IMSQRT", "IMSUB", "IMSUM", "IMTAN", "IMTANH", "INDEX", "INDIRECT", "INFO", "INT", "INTERCEPT", "INTRATE", "IPMT", "IRR", "ISBETWEEN", "ISBLANK", "ISDATE", "ISEMAIL", "ISERR", "ISERROR", "ISEVEN", "ISFORMULA", "ISLOGICAL", "ISNA", "ISNONTEXT", "ISNUMBER", "ISODD", "ISOMITTED", "ISOWEEKNUM", "ISPMT", "ISREF", "ISTEXT", "ISURL",
    "JOIN",
    "KURT",
    "LAMBDA", "LARGE", "LCM", "LEFT", "LEFTB", "LEN", "LENB", "LET", "LINEST", "LN", "LOG", "LOG10", "LOGEST", "LOGINV", "LOGNORM.DIST", "LOGNORM.INV", "LOGNORMDIST", "LOOKUP", "LOWER", "LT", "LTE",
    "MAKEARRAY", "MAP", "MARGINOFERROR", "MATCH", "MAX", "MAXA", "MAXIFS", "MDETERM", "MDURATION", "MEDIAN", "MID", "MIDB", "MIN", "MINA", "MINIFS", "MINUS", "MINUTE", "MINVERSE", "MIRR", "MMULT", "MOD", "MODE", "MODE.MULT", "MODE.SNGL", "MONTH", "MROUND", "MULTINOMIAL", "MULTIPLY", "MUNIT",
    "N", "NA", "NE", "NEGBINOM.DIST", "NEGBINOMDIST", "NETWORKDAYS", "NETWORKDAYS.INTL", "NOMINAL", "NORM.DIST", "NORM.INV", "NORM.S.DIST", "NORM.S.INV", "NORMDIST", "NORMINV", "NORMSDIST", "NORMSINV", "NOT", "NOW", "NPER", "NPV", "NUMBERVALUE",
    "OCT2BIN", "OCT2DEC", "OCT2HEX", "ODD", "ODDFPRICE", "ODDFYIELD", "ODDLPRICE", "ODDLYIELD", "OFFSET", "OR",
    "PDURATION", "PEARSON", "PERCENTIF", "PERCENTILE", "PERCENTILE.EXC", "PERCENTILE.INC", "PERCENTRANK", "PERCENTRANK.EXC", "PERCENTRANK.INC", "PERMUT", "PERMUTATIONA", "PHI", "PI", "PIVOTBY", "PMT", "POISSON", "POISSON.DIST", "POW", "POWER", "PPMT", "PRICE", "PRICEDISC", "PRICEMAT", "PROB", "PRODUCT", "PROPER", "PV",
    "QUARTILE", "QUARTILE.EXC", "QUARTILE.INC", "QUERY", "QUOTIENT",
    "RADIANS", "RAND", "RANDARRAY", "RANDBETWEEN", "RANK", "RANK.AVG", "RANK.EQ", "RATE", "RECEIVED", "REDUCE", "REGEXEXTRACT", "REGEXMATCH", "REGEXREPLACE", "REPLACE", "REPLACEB", "REPT", "RIGHT", "RIGHTB", "ROMAN", "ROUND", "ROUNDDOWN", "ROUNDUP", "ROW", "ROWS", "RRI", "RSQ",
    "SCAN", "SEARCH", "SEARCHB", "SEC", "SECH", "SECOND", "SEQUENCE", "SERIESSUM", "SHEET", "SHEETS", "SIGN", "SIN", "SINH", "SKEW", "SKEW.P", "SLN", "SLOPE", "SMALL", "SORT", "SORTBY", "SORTN", "SPARKLINE", "SPLIT", "SQRT", "SQRTPI", "STANDARDIZE", "STDEV", "STDEV.P", "STDEV.S", "STDEVA", "STDEVP", "STDEVPA", "STEYX", "STOCKHISTORY", "SUBSTITUTE", "SUBTOTAL", "SUM", "SUMIF", "SUMIFS", "SUMPRODUCT", "SUMSQ", "SUMX2MY2", "SUMX2PY2", "SUMXMY2", "SWITCH", "SYD",
    "T", "T.DIST", "T.DIST.2T", "T.DIST.RT", "T.INV", "T.INV.2T", "T.TEST", "TAKE", "TAN", "TANH", "TBILLEQ", "TBILLPRICE", "TBILLYIELD", "TDIST", "TEXT", "TEXTAFTER", "TEXTBEFORE", "TEXTJOIN", "TEXTSPLIT", "TIME", "TIMEVALUE", "TINV", "TO_DATE", "TO_DOLLARS", "TO_PERCENT", "TO_PURE_NUMBER", "TO_TEXT", "TOCOL", "TODAY", "TOROW", "TRANSPOSE", "TREND", "TRIM", "TRIMMEAN", "TRUE", "TRUNC", "TTEST", "TYPE",
    "UMINUS", "UNARY_PERCENT", "UNICHAR", "UNICODE", "UNIQUE", "UPLUS", "UPPER",
    "VALUE", "VALUETOTEXT", "VAR", "VAR.P", "VAR.S", "VARA", "VARP", "VARPA", "VDB", "VLOOKUP", "VSTACK",
    "WEBSERVICE", "WEEKDAY", "WEEKNUM", "WEIBULL", "WEIBULL.DIST", "WORKDAY", "WORKDAY.INTL", "WRAPCOLS", "WRAPROWS",
    "XIRR", "XLOOKUP", "XMATCH", "XNPV", "XOR",
    "YEAR", "YEARFRAC", "YIELD", "YIELDDISC", "YIELDMAT",
    "Z.TEST", "ZTEST",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtins() {
        assert!(is_builtin("SUMIFS") && is_builtin("XLOOKUP") && !is_builtin("MYFUNC"));
    }

    #[test]
    fn lexes_refs() {
        let f = "=SUMIFS('Revenue Input'!$C:$C,'Revenue Input'!$A:$A,$A4)+Monthly!B2:B+IMPORTRANGE(\"abc\",\"S!A1:B2\")+Rate*2";
        let lx = lex(f);
        let rs: Vec<String> = lx.refs.iter().map(|r| format!("{:?}!{}", r.sheet, r.rect().unwrap())).collect();
        assert_eq!(rs, vec![
            "Some(\"Revenue Input\")!C:C", "Some(\"Revenue Input\")!A:A", "None!A4", "Some(\"Monthly\")!B2:B"
        ]);
        assert_eq!(lx.funcs.iter().map(|f| f.name.as_str()).collect::<Vec<_>>(), vec!["SUMIFS", "IMPORTRANGE"]);
        assert_eq!(lx.names, vec!["Rate"]);
        let args = call_args(f, lx.funcs[1].open);
        assert_eq!(args, vec!["\"abc\"", "\"S!A1:B2\""]);
    }

    #[test]
    fn external_workbook_refs() {
        let lx = lex("='[Budget 2026.xlsx]Data'!B2+'[Rates.xlsx]'!Margin*C1");
        assert_eq!(lx.books, vec!["Budget 2026.xlsx", "Rates.xlsx"]);
        assert_eq!(lx.refs[0].sheet.as_deref(), Some("[Budget 2026.xlsx]Data"));
    }

    #[test]
    fn copies_share_key() {
        let a = "=year(B3)+$A$1";
        let b = "=year(C3)+$A$1";
        assert_eq!(r1c1_key(a, &lex(a), 0, 1), r1c1_key(b, &lex(b), 0, 2));
        let c = "=B4*C4";
        let d = "=B5*C5";
        assert_eq!(r1c1_key(c, &lex(c), 3, 3), r1c1_key(d, &lex(d), 4, 3));
    }

    #[test]
    fn thai_sheet_and_numbers() {
        let f = "=รายการ!B2*1E+5+1.5+LOG10(A1)";
        let lx = lex(f);
        assert_eq!(lx.refs.len(), 2);
        assert_eq!(lx.refs[0].sheet.as_deref(), Some("รายการ"));
        assert!(lx.names.is_empty(), "{:?}", lx.names);
    }
}
