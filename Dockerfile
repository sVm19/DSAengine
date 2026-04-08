# syntax=docker/dockerfile:1

FROM rust:1.92 AS builder
WORKDIR /app

COPY . .
RUN cargo build --release --bin dsaengine

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/dsaengine /app/dsaengine

EXPOSE 10000
CMD ["/app/dsaengine"]
