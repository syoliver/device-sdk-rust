ARG ALPINE_VERSION=3.20
ARG RUST_VERSION=1.79.0
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} as build_env

RUN apk add --no-cache musl-dev rust-gdb &&     \
        cargo install cargo-llvm-cov &&         \
        cargo install cargo-nextest &&          \
        rustup component add llvm-tools-preview

WORKDIR /src

FROM build_env as build_simulation
ARG BUILD_TYPE=debug #debug or release

COPY . .

RUN cd external/device-simulation && \
    if [ "$BUILD_TYPE" = "debug" ]; then cargo build; elif [ "$BUILD_TYPE" = "release" ]; then cargo build --release; fi


FROM alpine:${ALPINE_VERSION} as runtime
ARG BUILD_TYPE

COPY --from=build_simulation /src/target/${BUILD_TYPE}/device-simulation /usr/local/bin

CMD "device-simulation"



