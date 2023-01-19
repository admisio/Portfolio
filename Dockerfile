FROM rust:latest as builder
WORKDIR /portfolio
COPY . .
RUN cargo build --release
 
FROM debian:bookworm-slim
#RUN apt-get update && apt-get install -y PRIPADNE_DEPS && rm -rf /var/lib/apt/lists/*
COPY --from=builder /portfolio/target/release/portfolio /usr/local/bin/portfolio

VOLUME ["/portfolio"]
WORKDIR /portfolio

EXPOSE 8000
ENTRYPOINT ["portfolio"]
