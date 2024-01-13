FROM node:current-alpine
RUN apk add --no-cache gcc
RUN apk add --no-cache musl-dev
RUN apk add --no-cache rustup
RUN rustup toolchain install nightly
RUN rustup default nightly