FROM docker.io/library/rust:1.84.1-bullseye as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM docker.io/library/debian:bullseye-slim

COPY --from=builder /usr/src/app/target/release/secretsquirrel /usr/local/bin/secretsquirrel

CMD ["secretsquirrel"]
