#!/usr/bin/env bash
set -euxo pipefail
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner

cargo fmt --manifest-path=crate/yew_styles/Cargo.toml --all -- --check
cargo clippy --manifest-path=crate/yew_styles/Cargo.toml --all --all-features -- --deny=warnings
cargo test --target wasm32-unknown-unknown --all-features --manifest-path=crate/yew_styles/Cargo.toml
