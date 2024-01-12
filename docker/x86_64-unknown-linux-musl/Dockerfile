FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-alpine-zig
RUN apk add --no-cache libatomic
RUN rustup toolchain install nightly
RUN rustup default nightly