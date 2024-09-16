FROM rust:1.70 as builder
WORKDIR /usr/src/app
COPY server .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev netcat && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/collaborative-editor-server /usr/local/bin/collaborative-editor-server
ENV RUST_LOG=info
EXPOSE 8080
CMD ["collaborative-editor-server"]
