# Start from the official Rust image
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin actix-web-app
WORKDIR /actix-web-app

# Copy your project's Cargo.toml and Cargo.lock and build your project's dependencies
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Copy your source code into the Docker image and build your application
ADD . ./
RUN rm ./target/release/deps/actix_web_app*
RUN cargo build --release

# Start a new stage to create a lean production image
FROM debian:buster-slim
ARG APP=/usr/src/app

# Install necessary packages
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage and set permissions
COPY --from=builder /actix-web-app/target/release/actix-web-app ${APP}/actix-web-app

# Set the working directory and permissions
WORKDIR ${APP}
RUN chmod +x ${APP}/actix-web-app

# Run the binary
CMD ["./actix-web-app"]

## This Dockerfile uses a multi-stage build process to create a lean production image:

## 1. It starts from the official Rust image, builds your application, and compiles it in the first stage.
## 2. In the second stage, it uses a Debian Slim image, copies over the compiled application, and sets the necessary permissions.
