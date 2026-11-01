//! Excel number formats applied to numbers, so a cell reads as it shows on
//! screen (`25%`, `$12.50`, `(1,234)`) rather than as its stored value.
//!
//! Covers the common shapes: sections (`pos;neg;zero`), literals, `%`,
//! thousands separators and `,` scaling, fixed and optional decimals,
//! scientific notation, colour and currency tags. Anything else (dates,
//! fractions, conditions, digits mixed with literals) returns None, and the
//! caller keeps the plain value.

use crate::model::fmt_num;

/// `n` formatted with `code`, or None when the format isn't one this handles.
pub fn format(n: f64, code: &str) -> Option<String> {
    if !n.is_finite() {
        return None;
    }
    let sections = split_sections(code);
    if sections.iter().any(|s| s.contains("[>") || s.contains("[<") || s.contains("[=")) {
        return None;
    }
    let (sec, signed) = match (sections.len(), n) {
        (_, n) if n > 0.0 => (sections[0].as_str(), false),
        (1, n) if n < 0.0 => (sections[0].as_str(), true),
        (_, n) if n < 0.0 => (sections[1].as_str(), false),
        (l, _) if l >= 3 => (sections[2].as_str(), false),
        _ => (sections[0].as_str(), false),
    };
    let body = render(n.abs(), sec)?;
    Some(if signed && body.chars().any(|c| c.is_ascii_digit() && c != '0') { format!("-{body}") } else { body })
}

fn split_sections(code: &str) -> Vec<String> {
    let mut out = vec![String::new()];
    let mut chars = code.chars();
    while let Some(c) = chars.next() {
        match c {
            ';' => out.push(String::new()),
            '"' => {
                let last = out.last_mut().unwrap();
                last.push('"');
                for q in chars.by_ref() {
                    last.push(q);
                    if q == '"' {
                        break;
                    }
                }
            }
            '\\' => {
                let last = out.last_mut().unwrap();
                last.push('\\');
                last.extend(chars.next());
            }
            _ => out.last_mut().unwrap().push(c),
        }
    }
    out
}

enum Piece {
    Lit(String),
    Num,
}

/// One section: literals around a single run of digit placeholders.
fn render(v: f64, sec: &str) -> Option<String> {
    let mut pieces: Vec<Piece> = Vec::new();
    let mut pat = String::new(); // digit placeholders, '.', ',', 'E+00'
    let mut lit = String::new();
    let mut general = false;
    let mut percent = 0;
    let sec = replace_general(sec);
    let mut chars = sec.chars().peekable();
    let flush = |lit: &mut String, pieces: &mut Vec<Piece>| {
        if !lit.is_empty() {
            pieces.push(Piece::Lit(std::mem::take(lit)));
        }
    };
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                for q in chars.by_ref() {
                    if q == '"' {
                        break;
                    }
                    lit.push(q);
                }
            }
            '\\' => lit.extend(chars.next()),
            '_' => {
                chars.next();
                lit.push(' ');
            }
            '*' => {
                chars.next();
            }
            '[' => {
                let tag: String = chars.by_ref().take_while(|c| *c != ']').collect();
                // [$€-407] is a currency symbol; [$-409] only a locale; [Red] a colour.
                if let Some(cur) = tag.strip_prefix('$') {
                    lit.push_str(cur.split('-').next().unwrap_or(""));
                } else if tag.chars().next().is_some_and(|c| "hHmMsS".contains(c)) {
                    return None; // elapsed time
                }
            }
            '0' | '#' | '?' | '.' => {
                if pieces.iter().any(|p| matches!(p, Piece::Num)) {
                    return None; // digits broken up by literals (phone numbers, fractions)
                }
                flush(&mut lit, &mut pieces);
                pat.push(c);
                while let Some(&d) = chars.peek() {
                    if "0#?.,".contains(d) {
                        pat.push(d);
                        chars.next();
                    } else if (d == 'E' || d == 'e') && !pat.contains('E') {
                        chars.next();
                        let sign = chars.next().filter(|s| *s == '+' || *s == '-')?;
                        pat.push('E');
                        pat.push(sign);
                    } else {
                        break;
                    }
                }
                pieces.push(Piece::Num);
            }
            '%' => {
                percent += 1;
                lit.push('%');
            }
            GENERAL => {
                flush(&mut lit, &mut pieces);
                general = true;
                pieces.push(Piece::Num);
            }
            '@' | '/' => return None,
            c if c.is_ascii_alphabetic() && !matches!(c, 'E' | 'e') => {
                // y m d h s and AM/PM belong to dates and times.
                if "yYmMdDhHsSaA".contains(c) {
                    return None;
                }
                lit.push(c);
            }
            _ => lit.push(c),
        }
    }
    flush(&mut lit, &mut pieces);
    let v = v * 100f64.powi(percent);
    let num = if general {
        fmt_num(v)
    } else if pat.is_empty() {
        String::new()
    } else {
        number(v, &pat)
    };
    // `_)` and `* ` only pad for alignment.
    let out: String = pieces.into_iter().map(|p| match p {
        Piece::Lit(s) => s,
        Piece::Num => num.clone(),
    }).collect();
    Some(out.trim().to_string())
}

/// Stands in for the word `General` (outside quotes) while a section is read.
const GENERAL: char = '\u{1}';

fn replace_general(sec: &str) -> String {
    let mut out = String::new();
    let mut rest = sec;
    let mut quoted = false;
    while let Some(c) = rest.chars().next() {
        if !quoted && rest.get(..7).is_some_and(|w| w.eq_ignore_ascii_case("general")) {
            out.push(GENERAL);
            rest = &rest[7..];
            continue;
        }
        quoted ^= c == '"';
        out.push(c);
        rest = &rest[c.len_utf8()..];
    }
    out
}

/// `v` (not negative) through a placeholder pattern like `#,##0.00` or `0.0E+00`.
fn number(mut v: f64, pat: &str) -> String {
    let (mant, exp) = match pat.find('E') {
        Some(i) => (&pat[..i], Some(&pat[i + 1..])),
        None => (pat, None),
    };
    // Commas right after the last digit placeholder scale by 1000 each.
    let trimmed = mant.trim_end_matches(',');
    for _ in 0..mant.len() - trimmed.len() {
        v /= 1000.0;
    }
    let (int_pat, dec_pat) = trimmed.split_once('.').unwrap_or((trimmed, ""));
    let thousands = int_pat.contains(',');
    let min_int = int_pat.chars().filter(|c| *c == '0').count();
    let (min_dec, max_dec) = (dec_pat.chars().filter(|c| *c == '0' || *c == '?').count(), dec_pat.chars().filter(|c| "0#?".contains(*c)).count());

    let mut exponent = 0i32;
    if exp.is_some() && v != 0.0 {
        exponent = v.log10().floor() as i32;
        v /= 10f64.powi(exponent);
        // Rounding can carry the mantissa to 10.0.
        if format!("{v:.max_dec$}").starts_with("10") {
            v /= 10.0;
            exponent += 1;
        }
    }
    let fixed = format!("{v:.max_dec$}");
    let (int_s, dec_s) = fixed.split_once('.').unwrap_or((&fixed, ""));
    let mut dec = dec_s.to_string();
    while dec.len() > min_dec && dec.ends_with('0') {
        dec.pop();
    }
    let int_s = int_s.trim_start_matches('0');
    let mut int = format!("{:0>min_int$}", int_s);
    if thousands && int.len() > 3 {
        let mut grouped = String::new();
        for (i, ch) in int.chars().enumerate() {
            if i > 0 && (int.len() - i) % 3 == 0 {
                grouped.push(',');
            }
            grouped.push(ch);
        }
        int = grouped;
    }
    let mut out = int;
    if !dec.is_empty() || (dec_pat.is_empty() && trimmed.contains('.')) {
        out.push('.');
        out.push_str(&dec);
    }
    if let Some(e) = exp {
        let (sign, digits) = e.split_at(1);
        let width = digits.chars().filter(|c| *c == '0').count();
        let s = if exponent < 0 { "-" } else if sign == "+" { "+" } else { "" };
        out.push_str(&format!("E{s}{:0>width$}", exponent.abs()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::format;

    #[test]
    fn common_formats() {
        let f = |n: f64, code: &str| format(n, code);
        assert_eq!(f(0.25, "0%").as_deref(), Some("25%"));
        assert_eq!(f(0.1234, "0.0%").as_deref(), Some("12.3%"));
        assert_eq!(f(12.5, "$#,##0.00").as_deref(), Some("$12.50"));
        assert_eq!(f(1234567.0, "#,##0").as_deref(), Some("1,234,567"));
        assert_eq!(f(-1234.5, "$#,##0.00_);($#,##0.00)").as_deref(), Some("($1,234.50)"));
        assert_eq!(f(1234.5, "$#,##0.00_);($#,##0.00)").as_deref(), Some("$1,234.50"));
        assert_eq!(f(0.0, "_(* #,##0_);_(* \\(#,##0\\);_(* \"-\"??_);_(@_)").as_deref(), Some("-"));
        assert_eq!(f(-1500.0, "_(* #,##0_);_(* \\(#,##0\\);_(* \"-\"??_);_(@_)").as_deref(), Some("(1,500)"));
        assert_eq!(f(-5.0, "#,##0").as_deref(), Some("-5"));
        assert_eq!(f(0.0, "#,##0;(#,##0);\"-\"").as_deref(), Some("-"));
        assert_eq!(f(-7.0, "#,##0;[Red](#,##0)").as_deref(), Some("(7)"));
        assert_eq!(f(0.00012, "0.00E+00").as_deref(), Some("1.20E-04"));
        assert_eq!(f(123456.0, "0.0E+0").as_deref(), Some("1.2E+5"));
        assert_eq!(f(1_500_000.0, "0.0,,\"M\"").as_deref(), Some("1.5M"));
        assert_eq!(f(1234.5, "[$€-407] #,##0.00").as_deref(), Some("€ 1,234.50"));
        assert_eq!(f(5.0, "0.00").as_deref(), Some("5.00"));
        assert_eq!(f(0.5, "#.00").as_deref(), Some(".50"));
        assert_eq!(f(1.23456, "0.0#").as_deref(), Some("1.23"));
        assert_eq!(f(3.1, "0.0#").as_deref(), Some("3.1"));
        assert_eq!(f(42.0, "General\" units\"").as_deref(), Some("42 units"));
        assert_eq!(f(42.0, ";;;").as_deref(), Some(""));
        assert_eq!(f(-0.001, "0.0").as_deref(), Some("0.0"));
    }

    #[test]
    fn declines_what_it_cannot_render() {
        for code in ["yyyy-mm-dd", "h:mm", "# ?/?", "000-00-0000", "[>100]0;0.0", "@", "[h]:mm"] {
            assert_eq!(format(1.0, code), None, "{code}");
        }
    }
}
