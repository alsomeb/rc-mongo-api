# Use the official Rust image as the build environment
FROM rust:latest as builder

WORKDIR /usr/src/rc-mongo-api
COPY . .

# Build your application
RUN cargo build --release

# Start a new stage from the same Rust image
FROM rust:latest

# You might not need to install OpenSSL libraries separately since the Rust image might already include them,
# but if you do, here's how you could install additional runtime dependencies:
RUN apt-get update && apt-get install -y \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the binary from the builder stage
COPY --from=builder /usr/src/rc-mongo-api/target/release/rc-mongo-api .

# Copy the .env file
COPY --from=builder /usr/src/rc-mongo-api/.env .

# Make sure the binary is executable
RUN chmod +x rc-mongo-api

# Command to run the executable
CMD ["./rc-mongo-api"]

## This Dockerfile uses a multi-stage build process to create a lean production image:

## 1. It starts from the official Rust image, builds your application, and compiles it in the first stage.
## 2. In the second stage, it uses the same image, copies over the compiled application, and sets the necessary permissions.
