#!/bin/bash
# Regenerate .expect.html golden files from current binary output.
# Usage: bash _site/examples/gen.sh
set -euo pipefail

BIN="${1:-target/release/markdown-to-html}"
if [ ! -x "$BIN" ]; then
    echo "Building..."
    cargo build --release
    BIN="target/release/markdown-to-html"
fi

DIR="$(cd "$(dirname "$0")" && pwd)"
PASS=0
FAIL=0

for md in "$DIR"/*.md; do
    name=$(basename "$md" .md)
    out="$DIR/${name}.out.html"
    expect="$DIR/${name}.expect.html"

    "$BIN" "$md" > "$out" 2>/dev/null

    if [ -f "$expect" ]; then
        if diff -q "$out" "$expect" >/dev/null 2>&1; then
            echo "PASS  $name"
            PASS=$((PASS + 1))
        else
            echo "FAIL  $name"
            diff "$expect" "$out" || true
            FAIL=$((FAIL + 1))
        fi
    else
        echo "NEW   $name → $expect"
        cp "$out" "$expect"
    fi
    rm -f "$out"
done

echo ""
echo "Pass: $PASS  Fail: $FAIL"
