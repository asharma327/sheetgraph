# Spreadsheet map: ingredient-usage workbooks (TCM, EMQ, CSD)

You have structured extractions of three Google Sheets workbooks, one per branch (TCM, EMQ, CSD). Each has the same 23-tab layout and tracks raw-material usage. All three files are read-only snapshots, so nothing here touches the live sheets.

| dir | workbook | Google Sheet id |
|---|---|---|
| `TCM_1eAE0enn5I/` | ใบตรวจการใช้วัตถุดิบ (TCM) | 1eAE0enn5IpCKv0_rLAPuzI0kABWwsk9-nktymph0KbE |
| `EMQ_1C4jcPumRm/` | ใบตรวจการใช้วัตถุดิบ (EMQ) | 1C4jcPumRmTMZcmCdq5gVRdIu4wj60PhSegGDDCoudss |
| `CSD_1znYsQM9ue/` | ใบตรวจการใช้วัตถุดิบ (CSD) | 1znYsQM9ueZVxVbdTdrsnjbZuMSWhRu-z3oQZbwQjW5M |

## How to work

1. Read `INDEX.md`, then the `REPORT.md` of the workbook you need (about 25k tokens each). It is a summary map, not the data.
2. For exact cells, run the queries below. They read the local `workbook.json.gz`, which holds every cell, and need no network.
3. `graph.json` is the formula dependency graph, if you want to traverse it programmatically.

## Reading REPORT.md

- `ƒN` is formula copy-group N of a sheet. The formula is shown at the group's top-left cell and is copied over the range, with relative refs shifting. `↓` means filled down, `→` across, `▦` both.
- "same formula in N ranges … breaks at X = K hardcoded values" means a formula column was typed over in places. Those breaks are often where numbers stop matching expectations.
- "similar groups (same shape, refs/constants differ)" means the formulas have the same structure but point at different cells.
- "typed arithmetic" means hand-entered sums such as `+3820+3770`. They are data entry, not logic.
- `▸ range table …` is a data block. It lists one line per column with type and range, `enum` for small value sets, `fmt` for the number format, and `ƒN` when the column is computed. `[label]` in front of a header is the merged label above it, e.g. the ingredient name over its column group.
- `row-oriented` blocks have labels in the first column and one column per period (e.g. month).
- "same layout as X" means the tab is a structural clone of an earlier tab with different data. Use `sheet` (below) for the full section.
- Flags: `[external]` IMPORTRANGE and similar, `[dynamic ref]` INDIRECT/OFFSET (graph edges incomplete), `[array/spills]`, `[#REF!]`, `[N errors]`.

## Queries

Run from this folder. The binary is `bin/sheetgraph`, built for macOS arm64.

```bash
bin/sheetgraph cells TCM_1eAE0enn5I "'Monthly Summary (Usage)'!C1:H20"   # every cell: value, formula, format, validation, note
bin/sheetgraph trace TCM_1eAE0enn5I "Salmon!E4" --depth 2              # precedents (←) and dependents (→); --up / --down for one side
bin/sheetgraph find  EMQ_1C4jcPumRm "Yield"                             # search values, formulas, notes
bin/sheetgraph sheet CSD_1znYsQM9ue "Item Sales (Cal)" --full           # full untruncated section for one tab
bin/sheetgraph report TCM_1eAE0enn5I --full                             # whole report without truncation (large)
```

Quote sheet names that contain spaces or symbols: `"'Item Sales (Cal)'!A1:D10"`. Ranges are clamped to the sheet, and `--limit` raises the cell cap.

## Other workbooks these import from (not included here)

The `External links` section of each REPORT.md lists the exact ranges and the tabs they land in.

| id | workbook |
|---|---|
| 1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ | ใบนับของ (TCM2022): TCM daily stock count, half-month tabs `1/1`, `1/2`, … |
| 1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c | ใบนับของ (EMQ2022): EMQ daily stock count |
| 1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g | ใบนับของ (CSD2022): CSD daily stock count |
| 19pXjj0ZWWhr3kcWDRrT3w3Dxd88elcT1x1rKM1I1W6M | Revenue Consolidation: revenue / daily close (feeds the `Sales` tab) |
| 1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs, 1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o, 1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0 | not extracted (unknown title; likely newer stock-count files) |

## Known limits

- Apps Script, add-ons and triggers aren't visible through the Sheets API. Unknown function names would appear under "Non-builtin functions".
- Values are a snapshot taken 2026-10-23.
- Cells that receive an array formula's spill (e.g. from IMPORTRANGE) look like plain values. The formula sits in the top-left cell.
