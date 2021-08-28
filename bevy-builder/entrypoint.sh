#!/bin/bash

set -euo pipefail

cargo build --release
wasm-bindgen target/wasm32-unknown-unknown/release/bevy-project.wasm --target no-modules --no-typescript --remove-name-section --remove-producers-section --out-dir out