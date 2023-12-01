FROM rust:1.74.0 as builder

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller image to run the application
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/restaurant /usr/local/bin/restaurant

# Set the startup command
CMD ["restaurant"]