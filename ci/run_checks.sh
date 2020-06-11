#!/usr/bin/env bash
set -euxo pipefail

cargo fmt --manifest-path=crate/yew_styles/Cargo.toml --all -- --check
<<<<<<< HEAD
cargo clippy --manifest-path=crate/yew_styles/Cargo.toml --all -- --deny=warnings
wasm-pack test --headless --firefox crate
=======
cargo clippy --manifest-path=crate/yew_styles/Cargo.toml --all --all-features -- --deny=warnings
>>>>>>> development
