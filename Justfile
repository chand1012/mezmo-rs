set dotenv-load

default: build test

test:
    cargo test

build:
    cargo build --release
