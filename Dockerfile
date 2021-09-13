FROM rust:latest
ARG BUILD=/usr/src/app/build
ARG APP=/usr/src/app
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
RUN cd web && trunk build --release

WORKDIR ${APP}
RUN cp ${BUILD}/target/release/server .
RUN cp -R ${BUILD}/web/static .
RUN addgroup -S $APP_USER && adduser -S $APP_USER -G $APP_USER
RUN chown -R $APP_USER:$APP_USER ${BUILD}
RUN rm -Rf $APP

EXPOSE 9000
USER $APP_USER
CMD ["./server"]

