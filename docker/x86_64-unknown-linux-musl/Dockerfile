FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-alpine-zig
RUN apk add --nocache libatomic
RUN rustup toolchain install nightly
RUN rust default nightly