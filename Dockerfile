FROM rust:latest
LABEL authors="felix"

WORKDIR .
COPY . /app

RUN cargo build --release --manifest-path=/app/Cargo.toml