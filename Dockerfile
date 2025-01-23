# This will push compiled code for debian and linux like builds

FROM rust as builder
WORKDIR /usr/src/myapp

COPY . .

## get the repo from the copied "release_repo" file
RUN REPO=$(grep repo release_repo | cut -d '=' -f2) && \
    echo "REPO=$REPO" && \
    apt-get update && apt-get upgrade -y && apt-get install -y git

ENV REPO=$REPO

# compile the code for debian usage, 
RUN cargo install --path ./server_watchdog/
RUN cargo install --path ./installer/
RUN cargo install --path ./unistaller/

RUN git remote get-url origin || git remote add origin https://github.com/TommaruX3700/server_watchdog.git && \
    git fetch origin && \
    git checkout -b deb_release origin/deb_release


RUN git add . && \
    git commit -m "New debian release" && \
    git push --set-upstream origin deb_release

#clone the debian-release branch and push binaries to it

## this will setup a debian enviroment and run the program
# FROM debian:bookworm-slim
# RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/* && apt-get install libc6 && apt-get install git
# COPY --from=builder /usr/local/cargo/bin/server_watchdog /usr/local/bin/server_watchdog
# RUN git checkout -b 
# CMD ["server_watchdog"]