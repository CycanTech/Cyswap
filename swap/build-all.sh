#!/usr/bin/env bash

set -eu
cargo +nightly contract build --manifest-path contracts/core/base/psp22/Cargo.toml
cargo +nightly contract build --manifest-path contracts/core/base/weth9/Cargo.toml
cargo +nightly contract build --manifest-path contracts/core/pool/Cargo.toml
cargo +nightly contract build --manifest-path contracts/core/factory/Cargo.toml
cargo +nightly contract build --manifest-path contracts/periphery/NonfungiblePositionManager/Cargo.toml
