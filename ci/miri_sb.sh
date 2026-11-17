#!/bin/bash
set -euo pipefail

# Miri under Stacked Borrows, on one target.
#
# This script and `ci/miri_tb.sh` share target setup, but keep the aliasing models separate.
# rkyv's relative pointers require Tree Borrows, so only that lane enables rkyv.
#
# Usage: ci/miri_sb.sh <TARGET>

if [ -z "${1:-}" ]; then
  echo "Error: TARGET is not provided" >&2
  exit 1
fi

TARGET="$1"

# A C compiler and libc headers for the target. `cargo miri test` builds the dev-dependencies
# for TARGET, and criterion's `alloca` compiles C. Cross libc headers are only recommended by
# the compiler packages, so install them explicitly with --no-install-recommends.
if [ "$(uname)" = "Linux" ]; then
  packages=()
  case "$TARGET" in
    aarch64-unknown-linux-gnu) packages=(gcc-aarch64-linux-gnu libc6-dev-arm64-cross) ;;
    i686-unknown-linux-gnu) packages=(gcc-multilib) ;;
    powerpc64-unknown-linux-gnu) packages=(gcc-powerpc64-linux-gnu libc6-dev-ppc64-cross) ;;
    s390x-unknown-linux-gnu) packages=(gcc-s390x-linux-gnu libc6-dev-s390x-cross) ;;
    riscv64gc-unknown-linux-gnu) packages=(gcc-riscv64-linux-gnu libc6-dev-riscv64-cross) ;;
  esac
  if [ "${#packages[@]}" -gt 0 ]; then
    sudo apt-get update
    sudo apt-get install -y --no-install-recommends "${packages[@]}"
  fi
fi

rustup toolchain install nightly --profile minimal --component miri --component rust-src
rustup override set nightly
cargo miri setup

export MIRIFLAGS="-Zmiri-strict-provenance -Zmiri-disable-isolation -Zmiri-symbolic-alignment-check"

# rkyv cannot support Stacked Borrows: a reference to an archive header does not retain permission
# to read its out-of-line contents under that model. The integration remains covered by Tree
# Borrows, including its core-only path; do not disable aliasing checks or add pointer casts.
# https://github.com/rkyv/rkyv/issues/436#issuecomment-1767450952
# Derive the remaining feature list from Cargo so new integrations are covered automatically.
features=$(cargo metadata --no-deps --format-version 1 | python3 -c '
import json
import sys

package = next(p for p in json.load(sys.stdin)["packages"] if p["name"] == "cower")
print(",".join(sorted(set(package["features"]) - {"rkyv", "rkyv_0_8"})))
')
printf 'Stacked Borrows features: %s\n' "$features"
# The second pass covers the core tier, which enabling std/alloc does not compile.
#
# No `--all-targets`: it would interpret the criterion benches, which are measurement code and not
# code under test. The default selection runs the unit tests, the integration tests and the
# doctests, and only builds the examples.
cargo miri test --target "$TARGET" --features "$features"
cargo miri test --target "$TARGET" --no-default-features
