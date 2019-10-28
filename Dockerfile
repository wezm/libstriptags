FROM rust:1.38-slim

RUN cargo install cargo-deb

WORKDIR /src

VOLUME /out
