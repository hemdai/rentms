# Development Stage
FROM rust:1.83.0 AS builder

# Set the working directory
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release
FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend
EXPOSE 8080
CMD ["backend"]

# FROM rust:1.83.0-alpine
# RUN apk add musl-dev
# WORKDIR /app
# COPY ./Cargo.lock ./
# COPY ./Cargo.toml ./
# RUN mkdir src && echo "fn main() {}" > src/main.rs
# COPY ./src ./src
# RUN cargo build --release

# CMD ./target/release/lms