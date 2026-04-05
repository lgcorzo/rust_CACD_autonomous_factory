# Stage 1: Build the Rust workspace
FROM rust:1.94-slim-bookworm AS builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Build all workspace members
RUN cargo build --release --workspace

# Stage 2: Final runtime image
FROM debian:bullseye-slim

WORKDIR /usr/local/bin

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the binaries from the builder
COPY --from=builder /usr/src/app/target/release/factory-cli .
COPY --from=builder /usr/src/app/target/release/factory-mcp-server .

# Default command
CMD ["./factory-cli"]
