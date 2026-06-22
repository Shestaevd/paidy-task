FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM ubuntu:stonking-20260612

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY ./src/resources ./src/resources 

COPY --from=builder /app/target/release/paidy-home-task .
RUN chmod +x /app/paidy-home-task

CMD ["./paidy-home-task"]