# Base stage for cargo-chef
FROM rust:latest as chef
RUN cargo install cargo-chef
WORKDIR /app

# Planner stage
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this layer is cached as long as recipe.json hasn't changed
RUN cargo chef cook --release --recipe-path recipe.json
# Copy source and build
COPY . .
RUN cargo build --release

# Runtime Stage - use trixie for GLIBC 2.38 compatibility with rust:latest
FROM debian:trixie-slim
WORKDIR /app

# Install OpenSSL and curl (required for HTTP clients/servers and healthcheck)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates curl && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/almizan-core /app/almizan-core

# Copy static assets and templates
COPY --from=builder /app/static /app/static
COPY --from=builder /app/templates /app/templates

# Expose the application port
EXPOSE 3000

# Run the application
# Security: Create a specific user for the app
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Change ownership of the app directory
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Healthcheck to verify the application is running
HEALTHCHECK --interval=30s --timeout=3s \
    CMD curl -f http://localhost:3000/health || exit 1

CMD ["./almizan-core"]
