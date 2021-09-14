# Actix Yew Test

## Local Development

### Prerequisites

- Install `trunk` with `cargo install trunk wasm-bindgen-cli`
- Install new target with `rustup target add wasm32-unknown-unknown`

### Run locally

- Start the server with `cargo run --bin server` on port `8080`
- Start the frontend with  `cd web && trunk serve`

## Build

Run `build_docker.sh` in the root-directory to create a single `server`-docker image including the static output of
compiled `web`
