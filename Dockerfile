FROM alpine

RUN apk add --no-cache bash make cargo openssl-dev clang-dev clang g++	&& \
    rm -rf /var/cache/apk/*

WORKDIR /app
