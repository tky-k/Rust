FROM ubuntu:20.04

RUN apt-get update && apt-get --assume-yes install curl
# cargo installed automatically https://github.com/rust-lang/rustup/issues/297
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=${HOME}/.cargo/bin:${PATH}
# docker build -t rustup:1.0 .
# docker run --name rust_exsample -td rustup:1.0
# docker exec -it rust_exsample /bin/bash