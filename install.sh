#!/bin/sh
# Installs the latest sheetgraph release into ~/.local/bin (override with SHEETGRAPH_INSTALL_DIR).
#   curl -fsSL https://raw.githubusercontent.com/asharma327/sheetgraph/main/install.sh | sh
set -eu

repo="asharma327/sheetgraph"
dir="${SHEETGRAPH_INSTALL_DIR:-$HOME/.local/bin}"

case "$(uname -s)-$(uname -m)" in
  Darwin-arm64) target=aarch64-apple-darwin ;;
  Darwin-x86_64) target=x86_64-apple-darwin ;;
  Linux-x86_64) target=x86_64-unknown-linux-gnu ;;
  Linux-aarch64 | Linux-arm64) target=aarch64-unknown-linux-gnu ;;
  *)
    echo "sheetgraph: no prebuilt binary for $(uname -s) $(uname -m). Install with Rust instead: cargo install sheetgraph" >&2
    exit 1
    ;;
esac

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "Downloading sheetgraph for $target..."
curl -fsSL "https://github.com/$repo/releases/latest/download/sheetgraph-$target.tar.gz" | tar xz -C "$tmp"
mkdir -p "$dir"
mv "$tmp/sheetgraph-$target/sheetgraph" "$dir/sheetgraph"
chmod +x "$dir/sheetgraph"

echo "Installed $("$dir/sheetgraph" --version 2>/dev/null || echo sheetgraph) to $dir/sheetgraph"
case ":$PATH:" in
  *":$dir:"*) ;;
  *) echo "Add $dir to your PATH, e.g.: export PATH=\"$dir:\$PATH\"" ;;
esac
