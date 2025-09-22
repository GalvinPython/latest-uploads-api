FROM rust:1.90.0-alpine3.22 AS builder

# Install build dependencies for Rust + musl as we're using alpine
RUN apk add --no-cache \
    musl-dev \
    gcc \
    g++ \
    libc-dev \
    make \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

WORKDIR /usr/src/app

# Copy manifest files first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Build the app to cache dependencies
COPY . .
RUN cargo build --release --locked
RUN rm -rf src

FROM alpine:3.22

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the compiled binary from builder stage
COPY --from=builder /usr/src/app/target/release/server /usr/local/bin/app

# Expose default port
EXPOSE 8080

# Start the app
CMD ["app"]
