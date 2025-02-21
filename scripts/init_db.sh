#!/usr/bin/env bash
set -eo pipefail

# Load .env file
PARENT_DIR="$(dirname "$(pwd)")"
if [ -f "$PARENT_DIR/.env" ]; then
    export $(grep -v '^#' "$PARENT_DIR/.env" | xargs)
else
    echo ".env file not found in the parent directory."
    exit 1
fi

# Launch postgres using Docker
docker run \
-e POSTGRES_USER=${POSTGRES_USER} \
-e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
-e POSTGRES_DB=${POSTGRES_DB} \
-p ${POSTGRES_PORT}:${POSTGRES_PORT} \
-d postgres \
postgres -N 1000
# ^ Increased maximum number of connections for testing purposes

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${POSTGRES_PASSWORD}"
until psql -h "localhost" -U "${POSTGRES_USER}" -p "${POSTGRES_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${POSTGRES_PORT}!"

DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${POSTGRES_PORT}/${POSTGRES_DB}
export DATABASE_URL
sqlx database create