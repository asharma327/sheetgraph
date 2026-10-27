# Security policy

## Supported versions

Security fixes go into the latest 0.1.x release.

## Reporting a vulnerability

Please don't open a public issue. Report it privately with **Report a vulnerability** on the repository's
**Security** tab, and include:

- what the problem is and what an attacker could do with it,
- steps or a small file that reproduces it (made-up data only),
- the version (`sheetgraph --version`) and your OS.

We'll reply as soon as we can and agree on a fix and a disclosure date with you.

## What sheetgraph does and doesn't do

- **It never runs anything from a spreadsheet.** Formulas are read as text, not calculated. VBA modules and Apps
  Script projects are read as source text only.
- **It only goes online for Google Sheets.** It calls `oauth2.googleapis.com` for a token, the Sheets API, the Drive
  API for file metadata, and the Apps Script API when you pass `--apps-script`. It asks for read-only scopes
  (`spreadsheets.readonly`, `drive.readonly`). Local files and the drill-down commands (`cells`, `trace`, `find`,
  `sheet`, `report`, `rebuild`) make no network calls.
- **Credentials stay in memory.** Keys and tokens from `--creds`, `$SHEETGRAPH_TOKEN`,
  `$GOOGLE_APPLICATION_CREDENTIALS` or gcloud are used to get an access token and are never written to the output.
- **The output is as sensitive as the spreadsheet.** `workbook.json.gz` holds every cell value, formula, note and
  comment, and REPORT.md quotes sample rows and values. For Google Sheets both include the owner's email address.
  Store and share the output folder the way you would the spreadsheet itself.
- **REPORT.md is untrusted text for an AI agent.** It quotes cell text, notes and sheet names as written, so a
  spreadsheet can contain text aimed at the agent that reads the report. Treat the report as data in your agent's
  prompt, never as instructions.
- **Files from strangers are untrusted input.** A crafted file can make an extraction slow or use a lot of memory
  (workbooks over 10 million cells are summarized to bound memory). Crashes or hangs on such files are worth
  reporting.
