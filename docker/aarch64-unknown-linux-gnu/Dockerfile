FROM rust:slim-bookworm
RUN apt update
RUN apt install -y nodejs
RUN apt install -y yarn
RUN rustup toolchain install nightly
RUN rustup default nightly