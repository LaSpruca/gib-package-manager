#!/bin/fish
cd (dirname (status -f))
echo "Building Server"
cargo build --bin gib-server --release
wasm-pack build --target web --no-typescript
echo "Starting Server"
systemctl --user start gib-package-manager.service
