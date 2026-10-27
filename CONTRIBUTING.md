# Contributing

Bug reports, fixes and support for more spreadsheet features are welcome. For security problems, see
[SECURITY.md](SECURITY.md) instead of opening an issue.

## Reporting a bug

- **Never attach a real spreadsheet that holds confidential data**, and the same goes for its `REPORT.md` or
  `workbook.json.gz`: they contain the spreadsheet's values. Make the smallest copy with made-up data that still shows
  the problem.
- Say what you ran, what you expected and what you got, plus `sheetgraph --version`, your OS and the file type.

## Making a change

```bash
cargo test
cargo clippy --all-targets -- -D warnings
```

CI runs both on Linux, macOS and Windows, checks that the crate builds with Rust 1.88, and builds the docs with
warnings as errors.

- Add a test with every fix. Unit tests sit next to the code; end-to-end tests are in `tests/`.
- xlsx features are tested on `tests/fixtures/features.xlsx`. To cover a new one, add it to
  `tests/fixtures/make_features.py`, regenerate the file with openpyxl 3.1.5, and extend `tests/xlsx_features.rs`.
- Test data must be made up: never commit a real workbook, or output extracted from one.
- The code isn't formatted with rustfmt. Match the style around your change, and don't reformat code you aren't
  changing, so diffs stay readable.
- Add a line under `[Unreleased]` in [CHANGELOG.md](CHANGELOG.md) for anything users would notice.

By contributing, you agree that your contribution is licensed under the same terms as the project (MIT or
Apache-2.0, see [README.md](README.md#license)).
