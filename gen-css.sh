#!/bin/zsh
set -e

cargo r --bin generate_css
http website
