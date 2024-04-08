
set EDGEX_VERSION=v3.2.0-dev.12
curl --output device-sdk.yaml https://raw.githubusercontent.com/edgexfoundry/device-sdk-go/%EDGEX_VERSION%/openapi/v3/device-sdk.yaml
docker run --rm -v %CD%:/local openapitools/openapi-generator-cli:v7.4.0 generate -i /local/device-sdk.yaml -g rust-axum -o /local/src/api