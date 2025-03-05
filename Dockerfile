# Stage 1: Build the application
FROM rust:1.84 AS builder

RUN apt-get update && apt-get install -y libssl-dev pkg-config

WORKDIR /app
COPY . .

RUN cargo build --release

# Stage 2: Runtime environment
FROM debian:bookworm-slim

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Create app directory for config
WORKDIR /app
RUN mkdir -p configuration
COPY --from=builder /app/configuration/ /app/configuration/

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/areum-backend /usr/local/bin/areum-backend

ENV APP_ENVIRONMENT=production
# Set the entry point
CMD ["areum-backend"]