FROM rust:1.40 as builder

WORKDIR /app

RUN USER=root cargo init

COPY . .
RUN cargo build --release

FROM debian:stretch-slim

WORKDIR /bin

COPY --from=builder /app/target/release/cli .

RUN apt update && \
    apt install -y ca-certificates curl

CMD ["cli", "cost"]