# Stage 1: Build the application
FROM rust:1.84 AS builder

RUN apt-get update && apt-get install -y libssl-dev pkg-config

WORKDIR /app
COPY . .
RUN cargo install sqlx-cli --no-default-features --features postgres
RUN cargo build --release

# Stage 2: Runtime environment
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/areum-backend /usr/local/bin/areum-backend

# Set the entry point
CMD ["areum-backend"]