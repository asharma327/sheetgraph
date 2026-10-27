//! A1 addresses and rectangles. All coordinates are 0-based; rectangles are
//! inclusive. `OPEN` marks an unbounded edge (`A:A`, `1:1`, Sheets' `A2:A`).

use serde::{Deserialize, Serialize};

pub const OPEN: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Rect {
    pub r0: u32,
    pub c0: u32,
    pub r1: u32,
    pub c1: u32,
}

impl Rect {
    pub fn cell(r: u32, c: u32) -> Rect {
        Rect { r0: r, c0: c, r1: r, c1: c }
    }
    pub fn new(r0: u32, c0: u32, r1: u32, c1: u32) -> Rect {
        Rect { r0: r0.min(r1), c0: c0.min(c1), r1: r0.max(r1), c1: c0.max(c1) }
    }
    pub fn rows(&self) -> u64 {
        if self.r1 == OPEN { u64::MAX } else { (self.r1 - self.r0) as u64 + 1 }
    }
    pub fn cols(&self) -> u64 {
        if self.c1 == OPEN { u64::MAX } else { (self.c1 - self.c0) as u64 + 1 }
    }
    pub fn area(&self) -> u64 {
        self.rows().saturating_mul(self.cols())
    }
    pub fn contains(&self, r: u32, c: u32) -> bool {
        r >= self.r0 && r <= self.r1 && c >= self.c0 && c <= self.c1
    }
    pub fn intersects(&self, o: &Rect) -> bool {
        self.r0 <= o.r1 && o.r0 <= self.r1 && self.c0 <= o.c1 && o.c0 <= self.c1
    }
    pub fn union(&self, o: &Rect) -> Rect {
        Rect { r0: self.r0.min(o.r0), c0: self.c0.min(o.c0), r1: self.r1.max(o.r1), c1: self.c1.max(o.c1) }
    }
    /// Clamp open edges to a concrete grid size.
    pub fn clamp_to(&self, rows: u32, cols: u32) -> Rect {
        Rect {
            r0: self.r0.min(rows.saturating_sub(1)),
            c0: self.c0.min(cols.saturating_sub(1)),
            r1: self.r1.min(rows.saturating_sub(1)),
            c1: self.c1.min(cols.saturating_sub(1)),
        }
    }
    pub fn a1(&self) -> String {
        let whole_cols = self.r0 == 0 && self.r1 == OPEN;
        let whole_rows = self.c0 == 0 && self.c1 == OPEN;
        if whole_cols && !whole_rows {
            return format!("{}:{}", col_name(self.c0), col_name(self.c1));
        }
        if whole_rows && !whole_cols {
            return format!("{}:{}", self.r0 + 1, self.r1 + 1);
        }
        if self.r0 == self.r1 && self.c0 == self.c1 {
            return cell_name(self.r0, self.c0);
        }
        let end = match (self.r1 == OPEN, self.c1 == OPEN) {
            (true, false) => col_name(self.c1),
            (false, true) => format!("{}", self.r1 + 1),
            _ => cell_name(self.r1, self.c1),
        };
        format!("{}:{}", cell_name(self.r0, self.c0), end)
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.a1())
    }
}

pub fn col_name(mut c: u32) -> String {
    if c == OPEN {
        return String::new();
    }
    let mut s = Vec::new();
    loop {
        s.push(b'A' + (c % 26) as u8);
        if c < 26 {
            break;
        }
        c = c / 26 - 1;
    }
    s.reverse();
    String::from_utf8(s).unwrap()
}

pub fn cell_name(r: u32, c: u32) -> String {
    format!("{}{}", col_name(c), r + 1)
}

/// Column letters → 0-based index. Accepts up to 3 letters (Sheets allows ZZZ).
pub fn col_index(s: &str) -> Option<u32> {
    if s.is_empty() || s.len() > 3 || !s.bytes().all(|b| b.is_ascii_alphabetic()) {
        return None;
    }
    let mut n: u32 = 0;
    for b in s.bytes() {
        n = n * 26 + (b.to_ascii_uppercase() - b'A' + 1) as u32;
    }
    Some(n - 1)
}

/// Quote a sheet name for use in a reference when needed.
pub fn quote_sheet(name: &str) -> String {
    let plain = !name.is_empty()
        && name.chars().all(|ch| ch.is_alphanumeric() || ch == '_')
        && !name.chars().next().unwrap().is_ascii_digit()
        && parse_rect(name).is_none();
    if plain {
        name.to_string()
    } else {
        format!("'{}'", name.replace('\'', "''"))
    }
}

/// Parse `[Sheet!]A1[:B2]` (also `A:A`, `1:3`, `A2:A`). Returns (sheet, rect).
pub fn parse_ref(s: &str) -> Option<(Option<String>, Rect)> {
    let s = s.trim();
    let (sheet, rest) = match s.rfind('!') {
        Some(i) => {
            let sh = &s[..i];
            let sh = if sh.starts_with('\'') && sh.ends_with('\'') && sh.len() >= 2 {
                sh[1..sh.len() - 1].replace("''", "'")
            } else {
                sh.to_string()
            };
            (Some(sh), &s[i + 1..])
        }
        None => (None, s),
    };
    parse_rect(rest).map(|r| (sheet, r))
}

/// One endpoint of a reference: column and/or row, each optionally absolute.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pt {
    pub col: Option<u32>,
    pub row: Option<u32>,
    pub col_abs: bool,
    pub row_abs: bool,
}

pub fn parse_pt(s: &str) -> Option<Pt> {
    let b = s.as_bytes();
    let mut i = 0;
    let col_abs = b.first() == Some(&b'$');
    if col_abs {
        i += 1;
    }
    let cs = i;
    while i < b.len() && b[i].is_ascii_alphabetic() {
        i += 1;
    }
    let col = if i > cs { Some(col_index(&s[cs..i])?) } else { None };
    let mut row_abs = false;
    if i < b.len() && b[i] == b'$' {
        row_abs = true;
        i += 1;
    }
    let rs = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i != b.len() {
        return None;
    }
    let row = if i > rs {
        let n: u32 = s[rs..i].parse().ok()?;
        if n == 0 {
            return None;
        }
        Some(n - 1)
    } else {
        None
    };
    if col.is_none() && row.is_none() {
        return None;
    }
    // "$A" with no row and "$" before nothing are fine; "A$" alone is not.
    if row.is_none() && row_abs {
        return None;
    }
    if col.is_none() && col_abs {
        // "$5": the "$" makes the row absolute.
        return Some(Pt { col: None, row, col_abs: false, row_abs: true });
    }
    Some(Pt { col, row, col_abs, row_abs })
}

/// Combine two endpoints into a rectangle, filling open edges.
pub fn pts_rect(a: Pt, b: Option<Pt>) -> Option<Rect> {
    match b {
        None => Some(Rect::cell(a.row?, a.col?)),
        Some(b) => {
            match (a.col, a.row, b.col, b.row) {
                (Some(c0), Some(r0), Some(c1), Some(r1)) => Some(Rect::new(r0, c0, r1, c1)),
                (Some(c0), None, Some(c1), None) => Some(Rect { r0: 0, r1: OPEN, c0: c0.min(c1), c1: c0.max(c1) }),
                (None, Some(r0), None, Some(r1)) => Some(Rect { c0: 0, c1: OPEN, r0: r0.min(r1), r1: r0.max(r1) }),
                // Sheets open-ended: A2:B (to the bottom) / A2:2 (to the right)
                (Some(c0), Some(r0), Some(c1), None) => Some(Rect { r0, r1: OPEN, c0: c0.min(c1), c1: c0.max(c1) }),
                (Some(c0), None, Some(c1), Some(r1)) => Some(Rect { r0: r1, r1: OPEN, c0: c0.min(c1), c1: c0.max(c1) }),
                (Some(c0), Some(r0), None, Some(r1)) => Some(Rect { r0: r0.min(r1), r1: r0.max(r1), c0, c1: OPEN }),
                _ => None,
            }
        }
    }
}

pub fn parse_rect(s: &str) -> Option<Rect> {
    match s.split_once(':') {
        Some((a, b)) => pts_rect(parse_pt(a)?, Some(parse_pt(b)?)),
        None => pts_rect(parse_pt(s)?, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn names() {
        assert_eq!(col_name(0), "A");
        assert_eq!(col_name(25), "Z");
        assert_eq!(col_name(26), "AA");
        assert_eq!(col_name(701), "ZZ");
        assert_eq!(col_name(702), "AAA");
        assert_eq!(col_index("AAA"), Some(702));
        assert_eq!(parse_rect("A2:B").unwrap().a1(), "A2:B");
        assert_eq!(parse_rect("C:A").unwrap().a1(), "A:C");
        assert_eq!(parse_rect("$B$3:D10").unwrap().a1(), "B3:D10");
        assert_eq!(parse_rect("3:5").unwrap().a1(), "3:5");
        let (sh, r) = parse_ref("'Revenue Input'!A1:R9").unwrap();
        assert_eq!(sh.as_deref(), Some("Revenue Input"));
        assert_eq!(r.a1(), "A1:R9");
        assert_eq!(quote_sheet("Monthly"), "Monthly");
        assert_eq!(quote_sheet("Cash & QR"), "'Cash & QR'");
    }
}
