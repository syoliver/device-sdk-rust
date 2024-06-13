#!/bin/sh

CURRENT_DIR=$( cd "$( dirname "$0" )" && pwd )
. ${CURRENT_DIR}/.env

curl --output device-sdk.yaml https://raw.githubusercontent.com/edgexfoundry/device-sdk-go/${EDGEX_VERSION}/openapi/v3/device-sdk.yaml

docker run --rm -v ${CURRENT_DIR}:/local openapitools/openapi-generator-cli:latest  \
    generate                                                                        \
    -i /local/device-sdk.yaml                                                       \
    -t /local/tools/generator                                                       \
    -g rust-axum                                                                    \
    -c /local/openapi_config.yaml                                                   \
    -o /local/external/openapi                                                      \
    --generate-alias-as-model
