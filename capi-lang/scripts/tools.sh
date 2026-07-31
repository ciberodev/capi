#!/usr/bin/env sh
set -eu

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
cd "$script_dir/.."

required_rust="1.88.0"
required_cargo="1.88.0"
required_rustfmt="1.8.0-stable"
required_clippy="0.1.88"

rustc_version="$(rustc --version | awk '{print $2}')"
cargo_version="$(cargo --version | awk '{print $2}')"
rustfmt_version="$(rustfmt --version | awk '{print $2}')"
clippy_version="$(cargo clippy --version | awk '{print $2}')"

test "$rustc_version" = "$required_rust" || {
    echo "expected rustc $required_rust, found $rustc_version" >&2
    exit 1
}

test "$cargo_version" = "$required_cargo" || {
    echo "expected cargo $required_cargo, found $cargo_version" >&2
    exit 1
}

test "$rustfmt_version" = "$required_rustfmt" || {
    echo "expected rustfmt $required_rustfmt, found $rustfmt_version" >&2
    exit 1
}

test "$clippy_version" = "$required_clippy" || {
    echo "expected clippy $required_clippy, found $clippy_version" >&2
    exit 1
}

echo "rustc $rustc_version"
echo "cargo $cargo_version"
echo "rustfmt $rustfmt_version"
echo "clippy $clippy_version"
