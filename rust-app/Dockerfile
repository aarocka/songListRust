# Build stage
FROM rust:1.75-alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

# Runtime stage
FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/songlist-rust .

EXPOSE 8080

CMD ["./songlist-rust"]
