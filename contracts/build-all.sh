#!/usr/bin/env bash

set -eu
cargo +nightly contract build --manifest-path core/factory/Cargo.toml
cargo +nightly contract build --manifest-path periphery/pool/Cargo.toml
cargo +nightly contract build
