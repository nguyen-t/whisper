FROM node:alpine-latest AS node

FROM rust:alpine
COPY --from=node /usr/lib /usr/lib
COPY --from=node /usr/local/share /usr/local/share
COPY --from=node /usr/local/lib /usr/local/lib
COPY --from=node /usr/local/include /usr/local/include
COPY --from=node /usr/local/bin /usr/local/bin

RUN apk add --no-cache gcc
RUN apk add --no-cache musl-dev
RUN rustup toolchain install nightly
RUN rustup default nightly

