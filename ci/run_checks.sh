#!/usr/bin/env bash

set -euxo pipefail

cargo fmt --manifest-path=crate/yew_styles/Cargo.toml --all -- --check
cargo clippy --manifest-path=crate/yew_styles/Cargo.toml --all -- --deny=warnings
cargo test --target wasm32-unknown-unknown --all-features --manifest-path=crate/yew_styles/Cargo.toml
