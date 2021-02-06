#!/bin/bash

wasm-pack build --target web
rollup
RUST_LOG="actix_web=debug" PORT="4002" cargo run
