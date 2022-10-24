FROM rust:latest

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app

RUN cargo install cargo-watch

COPY ./ ./

RUN cargo build
