# Stage 1: Build
FROM rust:slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

# Stage 2: Runtime
FROM ubuntu:24.04

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/vtuber_song_list-cli ./
COPY --from=builder /app/config ./config
COPY --from=builder /app/assets ./assets

EXPOSE 5150

CMD ["./vtuber_song_list-cli", "start", "--environment", "production"]
