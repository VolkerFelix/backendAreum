# Stage 1: Build the application
FROM rust:1.84 AS builder
# Install system dependencies
RUN apt-get update && apt-get install -y libssl-dev pkg-config
# Create app directory and set it as working directory
WORKDIR /usr/src/app
# Copy source files and Cargo manifest
COPY src .
COPY Cargo.toml .
# Install SQLx CLI for database migrations
RUN cargo install sqlx-cli --no-default-features --features postgres
# Build the application in release mode
RUN cargo build --release

# Stage 2: Runtime environment
FROM debian:buster-slim

# Install system dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/areum-backend /usr/local/bin/areum-backend

# Set the entry point
CMD ["areum-backend"]