#!/usr/bin/env bash
set -x
set -eo pipefail
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT=
"${POSTGRES_PORT:=5432}"
# Launch postgres using Docker
docker run \
-e POSTGRES_USER=${POSTGRES_USER} \
-e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
-e POSTGRES_DB=${POSTGRES_DB} \
-p "${DB_PORT}":5432 \
-d postgres \
postgres -N 1000
# ^ Increased maximum number of connections for testing purposes