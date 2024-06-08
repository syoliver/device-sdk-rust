FOR /F "eol=# tokens=*" %%i IN (%~dp0.env) DO SET %%i

docker build                                        ^
    -t device-sdk-rust-build-env                    ^
    --target build_env                              ^
    --build-arg "ALPINE_VERSION=%ALPINE_VERSION%"   ^
    --build-arg "RUST_VERSION=%RUST_VERSION%"       ^
    .
docker run --rm -v %CD%:/src -it device-sdk-rust-build-env /bin/sh