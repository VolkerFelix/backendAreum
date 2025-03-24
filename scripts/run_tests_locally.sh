#!/bin/bash

# Function to check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        echo "Docker is not running. Please start Docker first."
        exit 1
    fi
}

# Function to start the test database
start_db() {
    echo "Starting test database..."
    docker-compose -f docker-compose.test.yml up -d
    echo "Waiting for database to be ready..."
    
    # Wait for PostgreSQL to be ready
    for i in {1..30}; do
        if docker exec backendareum-postgres-1 pg_isready -U postgres; then
            echo "PostgreSQL is ready."
            break
        fi
        echo "Waiting for PostgreSQL to be ready... (attempt $i/30)"
        sleep 2
    done
}

# Function to clean up the test database
cleanup_db() {
    echo "Cleaning up test database..."
    docker-compose -f docker-compose.test.yml down -v
}

# Main script
check_docker

# Start the test database
start_db

# Set environment variables for tests
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/areum_db"
export POSTGRES__DATABASE__USER=postgres
export POSTGRES__DATABASE__PASSWORD=postgres
export APP__APPLICATION__USER=app_user
export APP__APPLICATION__PASSWORD=app_password
export APP_ENVIRONMENT=local
export JWT_SECRET=test_jwt_secret_for_testing_only

# Create the test database and run migrations
echo "Creating database and running migrations..."
PGPASSWORD=postgres psql -U postgres -h localhost -c "DROP DATABASE IF EXISTS areum_db;"
PGPASSWORD=postgres psql -U postgres -h localhost -c "CREATE DATABASE areum_db;"
sqlx migrate run --database-url $DATABASE_URL

# Run the tests
echo "Running tests..."
cargo test

# Store the test exit code
TEST_EXIT_CODE=$?

# Clean up the database
cleanup_db

# Exit with the test exit code
exit $TEST_EXIT_CODE