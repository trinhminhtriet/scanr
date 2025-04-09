FROM rust:1.84.0-bookworm AS builder

# Install dependencies including LLVM 14 first
RUN apt-get update && apt-get install -y \
    clang \
    cmake \
    libssl-dev \
    pkg-config \
    llvm-14 \
    libclang-14-dev

# Set LIBCLANG_PATH after LLVM 14 is installed
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib

WORKDIR /app
COPY . /app

RUN cargo build --release


# FROM debian:bookworm-slim
FROM scratch
ARG APPLICATION="scanr"
ARG DESCRIPTION="ScanR: A lightweight, fast, and configurable port scanner built in Rust for reliable multi-platform network scanning."
ARG PACKAGE="trinhminhtriet/scanr"

LABEL org.opencontainers.image.ref.name="${PACKAGE}" \
    org.opencontainers.image.authors="Triet Trinh <contact@trinhminhtriet.com>" \
    org.opencontainers.image.documentation="https://github.com/${PACKAGE}/README.md" \
    org.opencontainers.image.description="${DESCRIPTION}" \
    org.opencontainers.image.licenses="MIT" \
    org.opencontainers.image.source="https://github.com/${PACKAGE}"

COPY --from=builder /app/target/release/scanr /bin/scanr
WORKDIR /workdir
ENTRYPOINT ["scanr"]
