FROM rust:1-slim-buster AS base

ENV USER=root

RUN apt update && apt install -y libghc-postgresql-libpq-dev pkg-config libssl-dev argon2 clang llvm-dev libclang-dev

WORKDIR /app

COPY . /code/

RUN cargo fetch
RUN cargo install diesel_cli --no-default-features --features postgres