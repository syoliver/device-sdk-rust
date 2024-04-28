ARG ALPINE_VERSION=3.19
ARG RUST_VERSION=1.77.2
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} as build_env
ARG BUILD_TYPE=debug #debug or release 

RUN apk add --no-cache musl-dev rust-gdb

WORKDIR /src

FROM build_env as build_simulation

COPY . .

RUN cd external/device-simulation && \
    if [ "$BUILD_TYPE" = "debug" ]; then cargo build; elif [ "$BUILD_TYPE" = "release" ]; then cargo build --release; fi


FROM alpine:${ALPINE_VERSION} as runtime
ARG BUILD_TYPE

COPY --from=build_simulation /src/target/${BUILD_TYPE}/device-simulation /usr/local/bin

CMD "device-simulation"



