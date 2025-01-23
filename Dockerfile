# This will push compiled code for debian and linux like builds

FROM rust as builder
WORKDIR /usr/src/myapp
COPY . .
RUN apt-get update && apt-get upgrade -y && apt-get install git
RUN cargo install --path ./server_watchdog/

# compile the code for debian usage, clone the debian-release branche and push binaries to it
## push here for debian binaries

## this will setup a debian enviroment
# FROM debian:bookworm-slim
# RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/* && apt-get install libc6 && apt-get install git
# COPY --from=builder /usr/local/cargo/bin/server_watchdog /usr/local/bin/server_watchdog
# RUN git checkout -b 


CMD ["server_watchdog"]