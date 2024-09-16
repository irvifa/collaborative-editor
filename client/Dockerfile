FROM rust:1.70 as builder
WORKDIR /usr/src/app
COPY client .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/collaborative-editor-client /usr/local/bin/collaborative-editor-client
ENV RUST_LOG=info
CMD ["collaborative-editor-client"]
