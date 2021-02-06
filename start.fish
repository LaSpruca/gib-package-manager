#!/bin/fish
cd (dirname (status -f))
echo "Building Server"
cargo install wasm-pack
cargo build --bin gib-server --release
cd gib-web
yarn install
yarn build
cd ..
echo "Starting Server"
systemctl --user start gib-package-manager.service
