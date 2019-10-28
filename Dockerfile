# Need to use stretch to match Ubuntu xenial in use on Travis
FROM rust:1.38-slim-stretch

RUN cargo install cargo-deb

WORKDIR /src

VOLUME /out
