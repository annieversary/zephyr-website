#!/bin/zsh
set -e

cargo watch -s './gen-css.sh' --why -w website/index.html
