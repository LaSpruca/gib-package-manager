#!/bin/fish
cd (dirname (status -f))
echo "Building Server"
cargo install wasm-pack
cargo install diesel_cli --no-default-features --features postgres
cargo build --bin gib-server --release
cd gib-server
diesel setup
diesel migration run
mkdir static
cd ..
cd gib-web
yarn install
yarn build
cd ..
echo "Starting Server"
systemctl --user start gib-package-manager.service
