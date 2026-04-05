# syntax=docker/dockerfile:1

FROM rust:1.92 AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Copy only the files needed to compute dependencies.
COPY Cargo.toml Cargo.lock ./
COPY macros/Cargo.toml ./macros/Cargo.toml
COPY src ./src
COPY macros ./macros

RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.92 AS builder
RUN cargo install cargo-chef
WORKDIR /app

COPY --from=chef /app/recipe.json recipe.json
COPY . .

RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/dsaengine /app/dsaengine

EXPOSE 3000
CMD ["/app/dsaengine"]
