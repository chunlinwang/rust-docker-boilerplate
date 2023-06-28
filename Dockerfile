FROM rust:alpine3.18 AS build

ENV USER=root

RUN apk update && apk add openssl postgresql14 build-base postgresql-dev

WORKDIR /build

RUN mkdir cert 
RUN openssl genrsa -out cert/private.pem 4096
RUN openssl rsa -in cert/private.pem -pubout -outform PEM -out cert/public.pem
RUN chmod 400 cert/*


COPY . /build/

RUN cargo fetch
RUN cargo install diesel_cli --no-default-features --features postgres


FROM rust:alpine3.18 AS production

ENV USER=root

RUN apk update && apk add openssl postgresql14 build-base postgresql-dev

WORKDIR /app

COPY --from=build /build/target/release /app/target/release
COPY --from=build /build/cert /app/cert