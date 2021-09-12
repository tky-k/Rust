FROM ubuntu:20.04

RUN apt-get update && apt-get --assume-yes install curl && apt-get --assume-yes install gcc
# cargo installed automatically https://github.com/rust-lang/rustup/issues/297
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=${HOME}/.cargo/bin:${PATH}
