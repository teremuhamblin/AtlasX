#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"
BIN="./cli/target/release/atlasx"

pushd cli > /dev/null
if [ ! -x "target/release/atlasx" ]; then
  echo "Compilation du binaire AtlasX..."
  cargo build --release
fi
popd > /dev/null

mkdir -p reports

"$BIN" --root "$ROOT" \
  --json "reports/atlasx-report.json" \
  --markdown "reports/atlasx-report.md" \
  --html "reports/atlasx-report.html"

echo "✅ Rapports générés dans ./reports"
