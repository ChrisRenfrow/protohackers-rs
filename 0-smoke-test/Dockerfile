FROM rust:latest AS builder

WORKDIR /usr/src/smoke-test
COPY . .
# Will build all dependent crates in release mode
RUN cargo build --release
# Build (install) the actual binaries
RUN cargo install --path .

# Runtime image
FROM debian:buster-slim
# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/local/cargo/bin/smoke-test /usr/local/bin/smoke-test
CMD ["smoke-test"]
