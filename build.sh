#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

build_dir="$(mktemp -d)"

clean() {
  rm -rf "$build_dir"
}

trap clean EXIT

# This will be done by build.rs of `server` anyway, but this way, we get nicer output
cargo build --manifest-path generator/Cargo.toml --release

GENERATOR_BUILD_DIRECTORY="${build_dir}" cargo build --manifest-path server/Cargo.toml --release --target x86_64-unknown-linux-musl
