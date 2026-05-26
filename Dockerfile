FROM rust:1-alpine3.20 AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

ENV OPENSSL_STATIC=1 \
    PKG_CONFIG_ALL_STATIC=1

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release && \
    cp target/release/rustyaestheticbot /usr/local/bin/rustyaestheticbot

FROM alpine:3.20 AS runtime

RUN apk add --no-cache ca-certificates && \
    adduser -D -H -s /sbin/nologin bot

COPY --from=builder /usr/local/bin/rustyaestheticbot /usr/local/bin/rustyaestheticbot

USER bot
WORKDIR /home/bot

ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/rustyaestheticbot"]
