#!/usr/bin/env bash

cargo fmt --manifest-path=crate/yew_styles/Cargo.toml --all -- --check
cargo clippy --manifest-path=crate/yew_styles/Cargo.toml --all -- --deny=warnings
cargo test --target wasm32-unknown-unknown --manifest-path=crate/yew_styles/Cargo.toml
