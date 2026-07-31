FROM rust:slim-bookworm AS rust-builder

WORKDIR /task_server

RUN apt update && apt install -y build-essential libssl-dev pkg-config 

COPY . .

RUN cargo build --bin task_server --release

FROM trion/ng-cli:20.3.3 AS angular-builder

WORKDIR /task_viewer

COPY --chmod=777 task_viewer .

RUN npm install && ng build

FROM debian:bookworm-slim

RUN useradd -m appuser
COPY --from=rust-builder /task_server/target/release/task_server /task_server
RUN mkdir -p /app/data
COPY --from=angular-builder /task_viewer/dist/task_viewer/browser /static
RUN chown appuser:appuser /task_server /static /app/data
USER appuser
# Run the binary
CMD ["/task_server"]
