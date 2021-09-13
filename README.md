# Actix Yew Test

## Local Development

### Prerequisites

- Install `trunk` with `cargo install trunk wasm-bindgen-cli`
- Install new target with `rustup target add wasm32-unknown-unknown`

### Run locally

- Start the server with `cargo run --bin server` on port `8080`
- Start the frontend with  `cd web && trunk serve`

## Build

Run `build.sh` in the root-directory to compile a single `server`
binary including the static output of compiled `web`, and package it into a docker container
