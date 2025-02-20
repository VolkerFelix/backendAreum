#!/usr/bin/env bash
set -x
set -eo pipefail

echo ${POSTGRES_USER}
echo ${POSTGRES_PASSWORD}
echo ${POSTGRES_DB}
echo ${POSTGRES_PORT}
# Launch postgres using Docker
docker run \
-e POSTGRES_USER=${POSTGRES_USER} \
-e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
-e POSTGRES_DB=${POSTGRES_DB} \
-p POSTGRES_PORT:${POSTGRES_PORT} \
-d postgres \
postgres -N 1000
# ^ Increased maximum number of connections for testing purposes