FROM rust:slim-bookworm AS rust-builder

WORKDIR /task_server

RUN apt update && apt install -y build-essential libssl-dev pkg-config 

COPY . .

RUN cargo build --bin consumer --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y curl \
    && useradd -m appuser

COPY --from=rust-builder /task_server/target/release/consumer /consumer

RUN chown appuser:appuser /consumer

USER appuser

# Run the binary
CMD ["/consumer"]
