ARG RUST_VERSION=1.73
ARG BUILD_TYPE="debug"

FROM rust:${RUST_VERSION} as builder

ARG BUILD_TYPE

RUN apt-get update && \
    apt-get -y install libelf-dev cmake bpftool gcc-multilib clang-16 linux-headers-generic && \
    ln -s /usr/bin/clang-16 /usr/bin/clang

COPY . /build/

WORKDIR /build

ENV BUILD_TYPE="$BUILD_TYPE"

RUN if [ "$BUILD_TYPE" = "debug" ]; then cargo build --examples; else cargo build --release --examples; fi
RUN ./scripts/build-bpf.sh

FROM ubuntu:jammy

ARG BUILD_TYPE

RUN apt-get update && \
    apt-get -y install gdb libelf-dev linux-tools-common

COPY --from=builder /build/target/$BUILD_TYPE/examples/falco-libbpf /usr/local/bin/
COPY --from=builder /build/target/$BUILD_TYPE/examples/falco /usr/local/bin/

RUN mkdir -p /drivers/

COPY --from=builder /build/cmake-build/main.bpf.o /drivers/

ENTRYPOINT ["bash"]
