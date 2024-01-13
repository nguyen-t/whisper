FROM rust:slim-bookworm
RUN apt update
RUN apt install -y gcc-arm-linux-gnueabihf
RUN apt install -y g++-arm-linux-gnueabihf
RUN apt install -y libclang-dev
RUN apt install -y nodejs
RUN apt install -y npm
RUN apt install -y snapd
RUN npm i -g yarn
RUN snap install zig --classic --beta
RUN rustup toolchain install nightly 
RUN rustup default nightly
ENV CC=gcc-arm-linux-gnueabihf
ENV CXX=g++-arm-linux-gnueabihf