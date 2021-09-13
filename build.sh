#!/usr/bin/env bash

echo -n "Adding dependencies"
cargo install trunk
rustup target add wasm32-unknown-unknown
rustup target add x86_64-unknown-linux-musl

echo -n "\n\nBuilding Actix-Yew-Test"

cd web
trunk build --release

cd ..
cargo build --bin server --release

mkdir -p target/release/bundle
cp target/release/server target/release/bundle
cp -R web/dist target/release/bundle/static

docker build -t actix-yew-test -f Dockerfile .

