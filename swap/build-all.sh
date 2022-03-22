#!/usr/bin/env bash

set -eu
cargo +nightly contract build --manifest-path contracts/core/pool/Cargo.toml
cargo +nightly contract build --manifest-path contracts/core/factory/Cargo.toml
# cargo +nightly contract build --manifest-path periphery/poolInitialize/Cargo.toml
