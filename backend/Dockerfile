# Use the official Rust image as the base
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the release version of the application
RUN cargo build --release

# Use a minimal runtime image
FROM ubuntu:22.04

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/analytics-platform-backend ./

# Expose the port your Axum app listens on
EXPOSE 3001

# Run the application
CMD ["./analytics-platform-backend"]
