FROM rust:1.75 as builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim
COPY --from=builder /usr/src/app/target/release/llm-rust /usr/local/bin/

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

CMD ["llm-rust"]
