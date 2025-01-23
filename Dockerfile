# This will push compiled code for debian and linux like builds

FROM rust as builder
WORKDIR /usr/src/myapp
COPY . .
## get the repo from the copied "release_repo" file
RUN REPO=$(grep repo ./release_repo | cut -d '=' -f2)
# ENV REPO=${REPO}

RUN apt-get update && apt-get upgrade -y && apt-get install git
# compile the code for debian usage, 
RUN cargo install --path ./server_watchdog/
RUN cargo install --path ./installer/
RUN cargo install --path ./unistaller/

RUN git remote add origin ${REPO} 
RUN fetch origin 
RUN checkout -b deb_release origin/deb_release 
RUN git add . 
RUN git commit -m "New debian release" 
RUN git push --set-upstream origin deb_release

#clone the debian-release branch and push binaries to it

## this will setup a debian enviroment and run the program
# FROM debian:bookworm-slim
# RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/* && apt-get install libc6 && apt-get install git
# COPY --from=builder /usr/local/cargo/bin/server_watchdog /usr/local/bin/server_watchdog
# RUN git checkout -b 
# CMD ["server_watchdog"]