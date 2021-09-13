FROM rust:latest AS builder
ARG BUILD=/usr/src/app

WORKDIR ${BUILD}
RUN rustup default nightly
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
RUN mkdir -p server/src && \
    mkdir -p web/src && \
    mkdir -p output
ADD ./Cargo.lock .
ADD ./Cargo.toml .
ADD ./server/Cargo.toml server/
RUN echo "fn main() {}" > server/src/main.rs
ADD ./web/Cargo.toml web/
RUN echo "fn main() {}" > web/src/main.rs
RUN cargo fetch

ADD ./server/src server/src
ADD ./web web
RUN cargo build --release --bin server
RUN cd web  && trunk build --release

FROM rust:1.55-slim
ARG APP=/usr/src/app
ENV APP_USER=appuser
WORKDIR $APP
COPY --from=builder $APP/web/dist $APP/static
COPY --from=builder $APP/target/release/server $APP

RUN addgroup --system $APP_USER && adduser --system --ingroup $APP_USER $APP_USER
RUN chown -R $APP_USER:$APP_USER $APP

EXPOSE 9000
USER $APP_USER

CMD ["./server"]


