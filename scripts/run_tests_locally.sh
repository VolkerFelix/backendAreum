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
    sleep 5
}

# Function to stop the test database
stop_db() {
    echo "Stopping test database..."
    docker-compose -f docker-compose.test.yml down
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
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres"
export POSTGRES__DATABASE__USER=postgres
export POSTGRES__DATABASE__PASSWORD=postgres
export POSTGRES__DATABASE__DB_NAME=postgres
export APP_ENVIRONMENT=local

# Run the tests
echo "Running tests..."
cargo test

# Store the test exit code
TEST_EXIT_CODE=$?

# Clean up the database
cleanup_db

# Exit with the test exit code
exit $TEST_EXIT_CODE 