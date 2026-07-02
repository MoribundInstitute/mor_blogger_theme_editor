#!/usr/bin/env bash
# Phase 16.4 smoke pass — the fastest full-confidence check without the GUI:
# workspace build + tests, then compile the fixture and every shipped preset
# through the real pipeline and assert the invariants that have bitten before.
set -euo pipefail
cd "$(dirname "$0")/.."

cargo build --workspace
cargo test --workspace
cargo build -p mor_blogger_cli --bin mbt

out=$(mktemp -d)
trap 'rm -rf "$out"' EXIT

./target/debug/mbt build -i build/fixtures/complex_workspace.toml -o "$out/fixture.xml"
grep -q "<b:skin" "$out/fixture.xml"

for f in theme_presets/*.toml; do
  name=$(basename "$f" .toml)
  ./target/debug/mbt build -i "$f" -o "$out/$name.xml"
  # every preset ships custom cursors (tested invariant; regressed once)
  grep -q "cursor: url(" "$out/$name.xml" || { echo "FAIL: $name has no custom cursor"; exit 1; }
done

# integrity gate: warnings allowed, errors are not
# (capture first: grep -q on a live pipe SIGPIPEs mbt mid-print)
report=$(./target/debug/mbt check -i build/fixtures/complex_workspace.toml)
echo "$report" | grep -Eq "0 error\(s\)|all checks passed"

echo "SMOKE OK"
