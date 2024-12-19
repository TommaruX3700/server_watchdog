# Creating standard image for the project

FROM rust as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path ./server_watchdog/

FROM debian:bookworm-slim
RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/* && apt-get install libc6
COPY --from=builder /usr/local/cargo/bin/server_watchdog /usr/local/bin/server_watchdog
CMD ["server_watchdog"]