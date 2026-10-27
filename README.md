# sheetgraph

> **Beta (0.1).** It works on real-world files, but the report format and the library API may still change between releases. Bug reports are welcome: see [CONTRIBUTING.md](CONTRIBUTING.md).

Extracts spreadsheets (Google Sheets, xlsx/xlsm/xls/xlsb/ods, csv) into a **compact map an LLM can read in one pass** plus a **formula dependency graph** — and saves the cells alongside it, so an agent can look up exact values when it needs to. Inspired by [graphify](https://github.com/Graphify-Labs/graphify) (overview report → scoped queries → full graph), but for spreadsheets.

Local files up to **10 million cells** are kept whole. Bigger ones still extract: the middle rows of their largest sheets are summarized per column (exact counts, min and max, most common values; no sums). Google Sheets are always read whole (see [Known gaps](#known-gaps)).

Made by [Love Spreadsheets](https://lovespreadsheets.com), which turns Google Sheets and Excel files into dashboards, CRMs, forms and other web apps with AI.

## Why it's small

A spreadsheet is mostly repetition. sheetgraph keeps the information and drops the repetition:

| Raw | Becomes |
|---|---|
| 3,541 cells `=MONTH(A2)`, `=MONTH(A3)`, … | one **copy-group**: `ƒ1 B2:B3542 ↓ =MONTH(A2)` (formulas grouped by R1C1 form, merged into rectangles) |
| the same lookup in 24 ranges interrupted by typed-over values | one line + *where it breaks and what's in the breaks* (`462 hardcoded values`) |
| 35k rows of data | header + per-column profile (type, range, enum values, number format, which ƒ computes it) + 4 sample rows |
| 2,000 date columns of the same shape | `D…BXZ (2,000 alike) "2021-09-01" … "2026-10-22": ƒ12; num …` |
| per-cell formats / validations | rectangles per distinct style / rule |

On an invoice ledger (120k cells, 32k formulas): **REPORT.md ≈ 7k tokens vs ≈ 1.3M tokens** for a plain cell dump.

## What a report looks like

An excerpt from `tests/fixtures/features.xlsx`:

```text
## 1. Data · A1:G21 · 127 cells · frozen 1r0c
read by: Summary
▸ A1:G21 table 21×7 (127 cells), header row 1
  A "Date": date 2026-01-05 … 2026-05-18 (20 filled)
  B "Region": enum East(5) North(5) South(5) West(5) (20 filled)
  E "Price": num 2.5 … 21.5 fmt $#,##0.00 (20 filled)
  F "Revenue": ƒ1 (20 filled)
ƒ1 F2:F21 ↓ =D2*E2
✓ validation
B2:B21: dropdown [North, South, East, West]
D2:D21: WHOLE_NUMBER_BETWEEN 0, 1000 (strict) help: "Units: Whole units, 0-1000"
◐ conditional formats
F2:F21: cell lessThan 0 → bg#FFC7CE fg#9C0006
table Sales A1:F21 [Date, Region, Product, Units, Price, Revenue]

## 3. Summary · A1:B11 · 13 cells · tab #FF0000
reads: Data (2g), Lists (1g)
ƒ1 B1 =SUM(Sales[Revenue])
ƒ4 B4 ='[Rates.xlsx]Rates'!B2*2 [external]
chart column "Revenue by week" ← 'Data'!F1, 'Data'!A2:A21, 'Data'!F2:F21
```

## Install

```bash
cargo install --git https://github.com/asharma327/spreadsheet-extractor
```

Or clone and `cargo build --release`; the binary is `target/release/sheetgraph`. Needs Rust 1.88 or newer.

## Usage

Extract a file, then read its report:

```bash
sheetgraph extract budget.xlsx      # writes sheetgraph-out/INDEX.md and sheetgraph-out/budget/
```

`sheetgraph-out/budget/REPORT.md` is the map: sheets, tables with column profiles, formula groups and how sheets feed each other. Pass several files (and Google Sheets) at once, and `-o <dir>` to write somewhere else.

```bash
# Google Sheets (ids or URLs) and/or local files, mixed
sheetgraph extract <id|url|file.xlsx>... -o sheetgraph-out --creds service-account.json

# drill down (all read workbook.json.gz; no network)
sheetgraph cells  sheetgraph-out/<wb> "'Revenue Input'!A1:F20"
sheetgraph trace  sheetgraph-out/<wb> "Monthly!E10" --depth 3   # precedents + dependents
sheetgraph find   sheetgraph-out/<wb> "VAT"
sheetgraph sheet  sheetgraph-out/<wb> "Ledger" --full           # untruncated section
sheetgraph report sheetgraph-out/<wb> [--full]                 # re-render without refetching
```

Local files need no credentials. For Google Sheets, credentials are tried in order: `--creds`, `$SHEETGRAPH_TOKEN` (raw access token), `$GOOGLE_APPLICATION_CREDENTIALS`, gcloud ADC (`gcloud auth application-default login`). Service accounts need the sheet shared with their email.

As a library:

```rust
let auth = sheetgraph::gsheets::Auth::discover(Some("sa.json"))?;
let wb = sheetgraph::load("<spreadsheet id or URL>", Some(&auth), false)?; // or a local path, with auth None
let a = sheetgraph::analyze::analyze(&wb);
let md = sheetgraph::report::render(&wb, &a, &Default::default());
let graph = sheetgraph::graph::build(&wb, &a); // serde_json::Value
```

## Output

```
sheetgraph-out/
  INDEX.md                    every workbook, sizes, links between workbooks (IMPORTRANGE, Excel external links)
  <workbook>/REPORT.md        the map — read this first
  <workbook>/graph.json       nodes (sheets, blocks, formula groups, charts, pivots, externals) + edges
  <workbook>/workbook.json.gz every cell (see Known gaps for very large files): value, formula, display text, style, validation, note, link
  <workbook>/meta.json
```

REPORT.md contains, per workbook: sheet list with used ranges and sheet→sheet data flow; external links (IMPORTRANGE / IMPORTDATA / … with targets resolved from cells, Excel links to other workbooks, refs to deleted sheets, Connected-Sheets data sources); named ranges; function census incl. **non-builtin functions** (Apps Script custom functions / Named Functions / VBA UDFs), dynamic refs (INDIRECT/OFFSET) and volatile functions; scripts (VBA modules from xlsm; Apps Script on request); developer metadata (left by add-ons/integrations).

Per sheet: blocks (tables with column profiles + samples, or small blocks verbatim), formula copy-groups and families with flags (`[array/spills]`, `[external]`, `[dynamic ref]`, `[#REF!]`, error counts), data validation (dropdowns, checkboxes, range-sourced lists), conditional formats, visual formatting regions, merges, filters/filter views, protected ranges, hidden rows/cols, banding, Excel tables, charts and pivots (with source ranges), notes and comments, hyperlinks.

## Giving it to an AI agent

Point the agent at `INDEX.md`, then the `REPORT.md` it needs, and let it run the drill-down commands for exact cells. Notation it should know:

- `ƒN` is formula copy-group N of a sheet. The formula is shown at the group's top-left cell and copied over the range with relative refs shifting: `↓` filled down, `→` across, `▦` both.
- "same formula in N ranges … breaks at X = K hardcoded values" means a formula column was typed over in places. Those breaks are often where numbers stop matching expectations.
- "typed arithmetic" is hand-entered sums like `=3820+3770`: data entry, not logic.
- `▸ range table …` is a data block, one line per column: type and range, `enum` for small value sets, `fmt` for the number format, `ƒN` when the column is computed.
- "same layout as X" means a sheet is a structural clone of an earlier one with different data; `sheetgraph sheet <dir> "<name>" --full` shows it in full.
- `⋯ rows A–B summarized` means a very large sheet's middle rows were summarized per column rather than kept: the column profiles still count every row, but sample rows and `cells` don't show those rows.
- Flags: `[external]` another file, `[dynamic ref]` INDIRECT/OFFSET (graph edges incomplete), `[array/spills]`, `[#REF!]`, `[N errors]`.

## Layout

| module | role |
|---|---|
| `model` | source-agnostic full-fidelity workbook (sparse cells, deduped styles/validations) |
| `gsheets` | auth (service account JWT / ADC refresh / token), chunked parallel grid fetch with retry, conversion |
| `xlsx` | calamine: values, formulas, defined names, merges, VBA; csv |
| `ooxml` | xlsx/xlsm package parts calamine skips: styles, validation, conditional formats, comments, hyperlinks, frozen/hidden rows and cols, filters, protection, tables, charts, pivots, external workbook links |
| `formula` | reference lexer (quoted/Thai sheet names, open-ended `A2:A`, whole rows/cols, `'[Book.xlsx]Sheet'`), R1C1 keys, function census |
| `analyze` | copy-groups, families, blocks, cross-sheet edges, externals |
| `report` | REPORT.md |
| `graph` | graph.json |
| `query` | cells / trace / find |
| `store` | on-disk layout, INDEX.md |

## Known gaps

- **Apps Script / macros / add-ons on Google Sheets**: the Sheets API doesn't expose bound scripts, installed add-ons or triggers, and the Apps Script API doesn't work with service accounts. `--apps-script <scriptId>` fetches a project with user OAuth (scope `script.projects.readonly`). Custom functions are still detected from formulas.
- **Local files**: xlsx/xlsm get the same coverage as Google Sheets, plus Excel tables and links to other workbooks (`'[Budget.xlsx]Data'!B2`). Images, shapes, sparklines and slicers aren't read (the report says when a file has them). xls, xlsb and ods get values, formulas, names and merges only.
- Number-format display text (e.g. `$1,234.50`) is only rendered for dates in local files; the format itself is on every cell.
- Spill ranges of array formulas are flagged on the formula but the spilled cells look like plain values.
- INDIRECT/OFFSET targets are dynamic, so those edges are missing (flagged in the report).
- **Very large local files**: up to 10 million cells are kept whole (about 0.2–0.4 GB of memory per million). Past that, a big sheet keeps its last 1,000 rows and its formulas, and summarizes the rows in between per column (counts, min and max, most common values; no sums), so memory stays near 2–3 GB however big the file is (a 160 MB xlsx with 44 million cells: 3 GB, 84 s). REPORT.md's column profiles still count every row and say which rows were summarized; `cells` only lists kept rows, and `find` reports text matches in summarized rows by column. The budget is `xlsx::CELL_BUDGET`; `xlsx::read_limited` takes another.
- **Very large Google Sheets** are read whole, without that budget. Google allows up to 20 million cells, so a sheet near the limit can take several GB of memory.

## Development

```bash
cargo test
```

`tests/fixtures/features.xlsx` exercises every xlsx feature the reader handles. It is generated by `tests/fixtures/make_features.py` (openpyxl 3.1.5); rerun the script after changing it.

See [CONTRIBUTING.md](CONTRIBUTING.md) before opening an issue or pull request, and [SECURITY.md](SECURITY.md) to report a vulnerability.

## Love Spreadsheets

sheetgraph comes from [Love Spreadsheets](https://lovespreadsheets.com). Describe what you need in a chat, and it builds a dashboard, CRM, form, calculator, project tracker or website from your Google Sheets and Excel files, with no coding.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
