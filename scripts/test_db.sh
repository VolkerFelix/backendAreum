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
case "$1" in
    "start")
        check_docker
        start_db
        ;;
    "stop")
        stop_db
        ;;
    "cleanup")
        cleanup_db
        ;;
    *)
        echo "Usage: $0 {start|stop|cleanup}"
        exit 1
        ;;
esac 