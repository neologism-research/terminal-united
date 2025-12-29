# Build stage
FROM rust:alpine3.23 AS builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev

# Copy the entire workspace (respecting .dockerignore)
COPY . .

# Build the server binary in release mode
RUN cargo build --release -p terminal-united-server

# Runtime stage
FROM alpine:3.23

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/terminal-united-server /usr/local/bin/terminal-united-server

# Expose the server port
EXPOSE 8080

# Set the entrypoint
CMD ["terminal-united-server"]
