# Build Stage
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# Build the application
# We build in release mode for maximum performance
RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

WORKDIR /app

# Install OpenSSL (often required for Rust HTTP clients/servers)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/backend /app/backend

# Expose the application port
EXPOSE 3000

# Run the application
CMD ["./backend"]
