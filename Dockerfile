## Builder
FROM rust:latest AS base
ARG BUILD=/usr/src/app

WORKDIR ${BUILD}

# Latest Rust and WASM target
RUN rustup default nightly
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

FROM base as compiler
WORKDIR ${BUILD}
RUN mkdir -p server/src && \
    mkdir -p web/src
ADD ./Cargo.lock .
ADD ./Cargo.toml .
ADD ./server/Cargo.toml server/
RUN echo "fn main() {}" > server/src/main.rs
ADD ./web/Cargo.toml web/
RUN echo "fn main() {}" > web/src/main.rs



FROM compiler as compiler_server
WORKDIR ${BUILD}
RUN cargo build --release --bin server
RUN rm -Rf target/release/deps/server-*
ADD ./server server
RUN cargo build --release --bin server

FROM compiler as compiler_web
WORKDIR ${BUILD}
RUN cargo build --release --bin web
RUN rm -Rf target/release/deps/web-*
ADD ./web/src web/src
ADD ./web/index.html web/
RUN cd web  && trunk build --release


## Runner
FROM rust:1.55-slim
ARG APP=/usr/src/app
ENV APP_USER=appuser
WORKDIR $APP

# Copy WASM output into static directory besides main binary
COPY --from=compiler_web $APP/web/dist $APP/static
COPY --from=compiler_server $APP/target/release/server $APP

# Make it safe and run
RUN addgroup --system $APP_USER && adduser --system --ingroup $APP_USER $APP_USER
RUN chown -R $APP_USER:$APP_USER $APP
EXPOSE 9000
USER $APP_USER
CMD ["./server"]


