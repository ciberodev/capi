#!/usr/bin/env sh
set -eu

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
cd "$script_dir/.."

test -f Cargo.lock

cargo metadata --no-deps --locked --format-version 1 >/dev/null
cargo tree --workspace --locked

if grep -q '^source = ' Cargo.lock; then
    echo "external dependency sources are not allowed by the current dependency policy" >&2
    echo "update DEPENDENCIES.md and scripts/deps.sh if an external dependency is approved" >&2
    exit 1
fi
