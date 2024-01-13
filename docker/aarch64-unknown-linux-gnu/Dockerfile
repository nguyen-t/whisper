FROM rust:slim-bookworm
RUN apt update
RUN apt install -y libclang-dev
RUN apt install -y nodejs
RUN apt install -y npm
RUN npm i -g yarn
RUN rustup toolchain install nightly 
RUN rustup default nightly