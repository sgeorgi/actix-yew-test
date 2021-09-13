FROM pactfoundation/rust-musl-build:latest as builder
ARG APP=/usr/src/app
WORKDIR ${APP}

RUN apk add build-base
RUN rustup default nightly
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add x86_64-unknown-linux-musl
RUN mkdir -p server/src && \
    mkdir -p web/src
ADD ./Cargo.lock .
ADD ./Cargo.toml .
ADD ./server/Cargo.toml server/
RUN echo "fn main() {}" > server/src/main.rs
ADD ./web/Cargo.toml web/
RUN echo "fn main() {}" > web/src/main.rs
RUN cargo fetch

ADD ./server/src server/src
ADD ./web web
RUN cargo build --release --bin server --target=x86_64-unknown-linux-musl
RUN cd web && trunk build --release

FROM alpine:latest
ARG APP=/usr/src/app
ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN addgroup -S $APP_USER && adduser -S $APP_USER -G $APP_USER

WORKDIR ${APP}
COPY --from=builder ${APP}/target/release/server ${APP}/
RUN chown -R $APP_USER:$APP_USER ${APP}

EXPOSE 9000
USER $APP_USER
CMD ["./server"]

