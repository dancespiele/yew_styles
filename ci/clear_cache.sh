#!/usr/bin/env bash
set -x

# inspired by https://github.com/rust-analyzer/rust-analyzer/blob/master/.travis.yml
find ./crate/yew_styles/target/debug -maxdepth 1 -type f -delete
find ./crate/yew_styles/target/tests/target/debug -maxdepth 1 -type f -delete
find ./crate/yew_styles/target/asmjs-unknown-emscripten/debug -maxdepth 1 -type f -delete
find ./crate/yew_styles/target/wasm32-unknown-emscripten/debug -maxdepth 1 -type f -delete
find ./crate/yew_styles/target/wasm32-unknown-unknown/debug -maxdepth 1 -type f -delete
rm -fr ./crate/yew_styles/target/debug/{deps,.fingerprint}/{*yew*,*\.was,*\.js*,*test*}
rm -fr ./crate/yew_styles/target/tests/target/debug/{deps,.fingerprint}/{*yew*,*\.was,*\.js*,*test*}
rm -fr ./crate/yew_styles/target/asmjs-unknown-emscripten/debug/{deps,.fingerprint}/{*yew*,*\.was,*\.js*,*test*}
rm -fr ./crate/yew_styles/target/wasm32-unknown-emscripten/debug/{deps,.fingerprint}/{*yew*,*\.was*,*\.js*,*test*}
rm -fr ./crate/yew_styles/target/wasm32-unknown-unknown/debug/{deps,.fingerprint}/{*yew*,*\.was*,*\.js*,*test*}
rm -fr ./crate/yew_styles/target/debug/incremental
rm -fr ./crate/yew_styles/target/tests/target/debug/incremental
rm -fr ./crate/yew_styles/target/asmjs-unknown-emscripten/debug/incremental
rm -fr ./crate/yew_styles/target/wasm32-unknown-emscripten/debug/incremental
rm -fr ./crate/yew_styles/target/wasm32-unknown-unknown/debug/incremental
rm -f  ./crate/yew_styles/target/.rustc_info.json
rm -f  ./crate/yew_styles/target/tests/target/.rustc_info.json
rm -fr ./crate/yew_styles/target/wasm32-unknown-unknown/wbg-tmp
rm -fr /home/travis/.cargo/registry/index/github.com-*