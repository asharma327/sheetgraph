# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Prebuilt binaries for macOS (Apple silicon and Intel), Linux (x86_64 and arm64) and Windows, attached to each
  GitHub release by `.github/workflows/release.yml`.
- `install.sh`, a one-line installer for macOS and Linux that downloads the latest release binary.
- Agent skill (`skills/sheetgraph/SKILL.md`) and a Claude Code plugin marketplace (`.claude-plugin/`), so agents run sheetgraph on their own when asked about a spreadsheet.
- README: before/after demo image (`assets/demo.png`) and the script that generates its sample workbook.
- Macros: each VBA procedure's trigger (workbook and sheet events, `Auto_Open`), the sheets and ranges it writes and
  uses, the procedures it calls, and the formula cells that call it as a function, in REPORT.md and as `macro` nodes
  with `writes` / `uses` / `calls` edges in graph.json. Apps Script functions are listed with their triggers.
- `sheetgraph macro <dir> [name]` lists macros or prints one module's or procedure's code; `find` also searches
  macro code.
- Sheets' VBA code names are read from xlsx/xlsm (`Sheet.code_name`).
- File-level settings: document and custom properties, calculation mode and iterative calculation (Excel and
  Google), and whether the sheet structure is locked, at the top of REPORT.md.
- Data connections (ODBC, OLE DB, web, text, Power Query) with passwords removed, and Power Query (M) code as
  `power_query` scripts that `macro` and `find` read.
- Pictures, shapes, text boxes and form controls over the grid (`Sheet.shapes`), with their text, alt text, the macro
  a button runs and a control's linked cell; graph.json gets `shape` nodes with `runs` and `writes` edges.
- Pictures placed in cells and Excel data types (Stocks, Geography) with their fields (`Cell.obj`); Google people and
  file chips the same way.
- Text with mixed formatting inside a cell (`Cell.runs`), from Excel and Google (`textFormatRuns`).
- Locked/unlocked cells and hidden formulas (`Style.unlocked`, `Style.hidden_formula`), listed per sheet as the
  inputs left editable under protection.
- Numbers in xlsx/xlsm show as their format displays them (`25%`, `$12.50`, `(1,234)`), not only dates.
- `tests/fixtures/audit.xlsx` and `tests/audit.rs`: one workbook with every item on the coverage checklist.

### Fixed

- A reference across sheets (`=SUM(Jan:Dec!B2)`) linked only the last sheet; it now links every sheet in the span.
- A reference to another workbook (`'[Other.xlsx]Data'!A1`) also produced a broken `reads` edge in graph.json.
- An xlsm whose VBA uses compression calamine doesn't expect no longer aborts the extraction; the macros are reported
  missing and everything else is kept.

### Changed

- REPORT.md counts modules with no code (the empty module Excel gives every sheet) instead of listing each one.

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
