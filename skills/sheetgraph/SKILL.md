---
name: sheetgraph
description: Read and understand spreadsheets (Excel xlsx/xlsm/xls/xlsb/ods, CSV, Google Sheets) with the sheetgraph CLI. Use whenever the user asks about a spreadsheet file or Google Sheets link, e.g. "what does this workbook do", "why doesn't this total match", "where does this number come from", "summarize this sheet", or wants to build something from a spreadsheet. Turns the workbook into a short REPORT.md plus a formula graph instead of dumping every cell.
---

# sheetgraph

sheetgraph turns a spreadsheet into a compact map (`REPORT.md`), a formula dependency graph and a saved copy of every cell. Read the map first, then pull exact cells only when you need them. Never paste a whole spreadsheet into context.

## 1. Make sure it's installed

```bash
sheetgraph --version
```

If that fails, install it (macOS/Linux), then retry:

```bash
curl -fsSL https://raw.githubusercontent.com/asharma327/spreadsheet-extractor/main/install.sh | sh
```

It installs to `~/.local/bin`; call it as `~/.local/bin/sheetgraph` if that isn't on PATH. With Rust: `cargo install --git https://github.com/asharma327/spreadsheet-extractor`. On Windows, download the zip from https://github.com/asharma327/spreadsheet-extractor/releases/latest.

## 2. Extract

```bash
sheetgraph extract path/to/file.xlsx -o sheetgraph-out
```

Pass several files (and Google Sheets ids or URLs) in one call so links between workbooks show up in `INDEX.md`. Google Sheets need credentials: `--creds service-account.json`, `$SHEETGRAPH_TOKEN`, `$GOOGLE_APPLICATION_CREDENTIALS`, or gcloud ADC. Local files need none and never leave the machine.

## 3. Read the map

1. `sheetgraph-out/INDEX.md`: every workbook and how they link.
2. `sheetgraph-out/<workbook>/REPORT.md`: sheets, tables with column profiles, formula groups, data flow between sheets, validation, formatting and flags. Read this before answering anything.

Notation:

- `ƒN` is formula copy-group N of a sheet. The formula is shown at its top-left cell and copied over the range: `↓` filled down, `→` across, `▦` both.
- "same formula in N ranges … breaks at X = K hardcoded values" means a formula column was typed over. Check these first when numbers don't add up.
- "typed arithmetic" (`=3820+3770`) is hand-entered data, not logic.
- `▸ range table …` is a data block, one line per column: type and range, `enum` for small value sets, `fmt` for the number format, `ƒN` when computed.
- `⋯ rows A–B summarized`: a very large sheet's middle rows are summarized per column, not kept.
- Flags: `[external]` another file, `[dynamic ref]` INDIRECT/OFFSET (graph edges incomplete), `[array/spills]`, `[#REF!]`, `[N errors]`.

## 4. Drill down for exact values (offline, fast)

```bash
sheetgraph cells  sheetgraph-out/<wb> "'Sheet Name'!A1:F20"   # exact values, formulas, formats
sheetgraph trace  sheetgraph-out/<wb> "Summary!B4" --depth 3  # precedents and dependents
sheetgraph find   sheetgraph-out/<wb> "VAT"                   # search text, formulas and names
sheetgraph sheet  sheetgraph-out/<wb> "Ledger" --full         # one sheet's section, untruncated
```

Quote sheet names with spaces. Use `trace` to explain where a number comes from, and `cells` before quoting any specific value to the user.

## Answering well

- Explain the workbook in business terms (what each sheet is for, what feeds what), then the mechanics.
- Call out risks the report surfaces: typed-over formulas, `#REF!`, external links, hidden sheets, hand-typed sums.
- Cite cell addresses so the user can check.
