FROM  rust:1.95-alpine as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock
COPY src/ .
RUN cargo build --release
FROM alpine:3.23.4
COPY --from=builder  /app/target/release/todo-cli-rust /usr/local/bin/todo-cli-rust
ENTRYPOINT ["/usr/local/bin/todo-cli-rust"]

