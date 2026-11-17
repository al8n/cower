#!/bin/bash
set -euo pipefail

# Miri under Tree Borrows, on one target.
#
# This script and `ci/miri_sb.sh` share target setup, but keep the aliasing models separate.
# This lane includes rkyv, whose relative pointers require Tree Borrows.
#
# Usage: ci/miri_tb.sh <TARGET>

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

export MIRIFLAGS="-Zmiri-strict-provenance -Zmiri-disable-isolation -Zmiri-symbolic-alignment-check -Zmiri-tree-borrows"

# Cover all features and the bare core tier. The final pass also exercises rkyv deserialization
# directly into heapless storage: neither of the first two configurations compiles that path.
#
# No `--all-targets`: it would interpret the criterion benches, which are measurement code and not
# code under test. The default selection runs the unit tests, the integration tests and the
# doctests, and only builds the examples.
cargo miri test --target "$TARGET" --all-features
cargo miri test --target "$TARGET" --no-default-features
cargo miri test --target "$TARGET" --no-default-features --features heapless,rkyv
