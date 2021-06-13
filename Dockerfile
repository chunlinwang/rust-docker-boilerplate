FROM rust:alpine AS base

ENV USER=root

# RUN apk update -y && apk add -y libghc-postgresql-libpq-dev \
# pkg-config libssl-dev argon2 clang llvm-dev libclang-dev \
# libxcb-shape0-dev libxcb-xfixes0-dev

WORKDIR /code
RUN cargo init
COPY . /code/

RUN cargo fetch
