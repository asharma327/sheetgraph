# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0]

First public release, as a beta.

### Added

- `sheetgraph extract` for Google Sheets (ids or URLs) and local xlsx, xlsm, xls, xlsb, ods and csv files, writing
  `REPORT.md`, `graph.json`, `workbook.json.gz` and `meta.json` per workbook, plus an `INDEX.md` across workbooks.
- Drill-down commands that work offline on a saved extraction: `cells`, `trace`, `find`, `sheet`, `report`, `rebuild`.
- Formula copy-groups and families, with breaks where a copied formula was typed over, and hand-typed arithmetic.
- Data blocks with per-column profiles, cross-sheet data flow, structural clones of sheets, and a function census
  that flags non-builtin, dynamic and volatile functions.
- Google Sheets: formatting, data validation, conditional formats, notes, hyperlinks, filters and filter views,
  protected ranges, banding, charts, pivots, Connected Sheets data sources, developer metadata, IMPORTRANGE links,
  and Apps Script projects with `--apps-script`.
- xlsx/xlsm: formatting with resolved theme colours, number formats, data validation, conditional formats, comments
  (including threaded replies), hyperlinks, frozen panes, tab colours, hidden rows and columns, filters, sheet
  protection, Excel tables, charts, pivots, VBA modules, and links to other workbooks (`'[Budget.xlsx]Data'!B2`).
- Excel tables resolve in the dependency graph, so `Sales[Amount]` counts as reading the table's sheet.
- INDEX.md connects workbooks extracted together through IMPORTRANGE or Excel external links.
- Local workbooks of any size: xlsx is read cell by cell, every cell is kept up to 10 million, and past that big sheets
  keep their last rows and formulas and summarize the rest per column (counts, ranges, most common values), so memory
  stays bounded while the report's column profiles still count every row.
