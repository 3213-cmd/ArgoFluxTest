FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml ./
COPY config.toml ./
COPY content.txt ./
COPY src ./src
COPY templates ./templates

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/server ./server
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/config.toml ./config.toml
COPY --from=builder /app/content.txt ./content.txt

EXPOSE 3000

ENV PORT=3000
ENV RUST_LOG=info

CMD ["./server"]