#!/usr/bin/env sh
set -eu

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
cd "$script_dir/.."

scripts/tools.sh
scripts/fmt-check.sh
scripts/deps.sh
scripts/lint.sh
scripts/test.sh
scripts/build.sh
scripts/doc.sh

cargo run -p capi-cli --locked -- --help
cargo run -p capi-cli --locked -- --version
cargo run -p capi-cli --locked -- --emit tokens tests/lexer/pass/basic.cap

set +e
output="$(cargo run -p capi-cli --locked -- does-not-exist.capi 2>&1)"
status="$?"
set -e

test "$status" -ne 0
printf '%s\n' "$output" | grep "error: failed to read source file 'does-not-exist.capi'"
