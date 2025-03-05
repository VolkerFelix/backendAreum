#!/bin/bash
set -e

if [ "$APP_ENVIRONMENT" = "production" ]; then
  echo "Production environment detected. Running database migrations..."
  sqlx migrate run
else
  echo "Non-production environment. Skipping automatic migrations."
fi

echo "Starting application..."
exec areum-backend