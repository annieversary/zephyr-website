#!/bin/zsh
set -e

wasm-pack build --release \
              --target web \
              --out-dir website/wasm

cargo r --bin generate_css

if [ ! "$1" = "-r" ]; then
http website
fi
