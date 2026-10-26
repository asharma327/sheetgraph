# sheetgraph

Extracts spreadsheets (Google Sheets, xlsx/xlsm/xls/ods, csv) into a **compact map an LLM can read in one pass** plus a **formula dependency graph** — and keeps every cell on disk so an agent can drill down exactly when it needs to. Inspired by [graphify](https://github.com/Graphify-Labs/graphify) (overview report → scoped queries → full graph), but for spreadsheets.

## Why it's small

A spreadsheet is mostly repetition. sheetgraph keeps the information and drops the repetition:

| Raw | Becomes |
|---|---|
| 3,541 cells `=MONTH(A2)`, `=MONTH(A3)`, … | one **copy-group**: `ƒ1 B2:B3542 ↓ =MONTH(A2)` (formulas grouped by R1C1 form, merged into rectangles) |
| the same lookup in 24 ranges interrupted by typed-over values | one line + *where it breaks and what's in the breaks* (`462 hardcoded values`) |
| 35k rows of data | header + per-column profile (type, range, enum values, number format, which ƒ computes it) + 4 sample rows |
| 2,000 date columns of the same shape | `D…BXZ (2,000 alike) "2021-09-01" … "2026-10-22": ƒ12; num …` |
| per-cell formats / validations | rectangles per distinct style / rule |

On the invoice ledger (120k cells, 32k formulas): **REPORT.md ≈ 7k tokens vs ≈ 1.3M tokens** for a plain cell dump.

## Output

```
sheetgraph-out/
  INDEX.md                    every workbook, sizes, cross-workbook IMPORTRANGE links
  <workbook>/REPORT.md        the map — read this first
  <workbook>/graph.json       nodes (sheets, blocks, formula groups, charts, pivots, externals) + edges
  <workbook>/workbook.json.gz every cell: value, formula, display text, style, validation, note, link
  <workbook>/meta.json
```

REPORT.md contains, per workbook: sheet list with used ranges and sheet→sheet data flow; external links (IMPORTRANGE / IMPORTDATA / … with targets resolved from cells, refs to deleted sheets, Connected-Sheets data sources); named ranges; function census incl. **non-builtin functions** (Apps Script custom functions / Named Functions / VBA UDFs), dynamic refs (INDIRECT/OFFSET) and volatile functions; scripts (VBA modules from xlsm; Apps Script on request); developer metadata (left by add-ons/integrations).

Per sheet: blocks (tables with column profiles + samples, or small blocks verbatim), formula copy-groups and families with flags (`[array/spills]`, `[external]`, `[dynamic ref]`, `[#REF!]`, error counts), data validation (dropdowns, checkboxes, range-sourced lists), conditional formats, visual formatting regions, merges, filters/filter views, protected ranges, hidden rows/cols, banding, charts and pivots (with source ranges), notes, hyperlinks.

## Usage

```bash
cargo build --release

# Google Sheets (ids or URLs) and/or local files, mixed
./target/release/sheetgraph extract <id|url|file.xlsx>... -o sheetgraph-out --creds service-account.json

# drill down (all read workbook.json.gz; no network)
./target/release/sheetgraph cells  sheetgraph-out/<wb> "'Revenue Input'!A1:F20"
./target/release/sheetgraph trace  sheetgraph-out/<wb> "Monthly!E10" --depth 3   # precedents + dependents
./target/release/sheetgraph find   sheetgraph-out/<wb> "VAT"
./target/release/sheetgraph sheet  sheetgraph-out/<wb> "Ledger" --full           # untruncated section
./target/release/sheetgraph report sheetgraph-out/<wb> [--full]                 # re-render without refetching
```

Credentials, in order: `--creds`, `$SHEETGRAPH_TOKEN` (raw access token), `$GOOGLE_APPLICATION_CREDENTIALS`, gcloud ADC (`gcloud auth application-default login`). Service accounts need the sheet shared with their email.

As a library:

```rust
let auth = sheetgraph::gsheets::Auth::discover(Some("sa.json"))?;
let wb = sheetgraph::load("19pXjj0Z…", Some(&auth), false)?;      // or a path
let a = sheetgraph::analyze::analyze(&wb);
let md = sheetgraph::report::render(&wb, &a, &Default::default());
let graph = sheetgraph::graph::build(&wb, &a);                     // serde_json::Value
```

## Layout

| module | role |
|---|---|
| `model` | source-agnostic full-fidelity workbook (sparse cells, deduped styles/validations) |
| `gsheets` | auth (service account JWT / ADC refresh / token), chunked parallel grid fetch with retry, conversion |
| `xlsx` | calamine: values, formulas, defined names, merges, VBA; csv |
| `formula` | reference lexer (quoted/Thai sheet names, open-ended `A2:A`, whole rows/cols), R1C1 keys, function census |
| `analyze` | copy-groups, families, blocks, cross-sheet edges, externals |
| `report` | REPORT.md |
| `graph` | graph.json |
| `query` | cells / trace / find |

## Known gaps

- **Apps Script / macros / add-ons on Google Sheets**: the Sheets API doesn't expose bound scripts, installed add-ons or triggers, and the Apps Script API doesn't work with service accounts. `--apps-script <scriptId>` fetches a project with user OAuth (scope `script.projects.readonly`). Custom functions are still detected from formulas.
- **Local xlsx**: formatting, data validation, conditional formats and charts aren't extracted yet (calamine doesn't read them); next step is reading those parts of the xlsx XML directly.
- Spill ranges of array formulas are flagged on the formula but the spilled cells look like plain values.
- INDIRECT/OFFSET targets are dynamic, so those edges are missing (flagged in the report).
