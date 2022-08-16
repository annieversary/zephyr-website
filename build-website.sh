#!/bin/zsh
set -e

wasm-pack build --release \
              --target web \
              --out-dir website/wasm

cargo r --bin generate_css

http website
