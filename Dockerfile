FROM alpine

RUN apk add --no-cache bash make rustup openssl-dev clang-dev clang g++	&& \
    cargo-init -y && \
    rm -rf /var/cache/apk/*

WORKDIR /app
