docker build -t device-sdk-rust-build-env --target build_env .
docker run --rm -v %CD%:/src -it device-sdk-rust-build-env /bin/sh