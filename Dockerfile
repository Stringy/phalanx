ARG RUST_VERSION=1.73

FROM rust:${RUST_VERSION}

RUN apt-get update && apt-get -y install gdb libelf-dev cmake bpftool clang

ENTRYPOINT ["bash", "-c"]

