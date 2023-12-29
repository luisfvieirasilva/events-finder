#!/bin/bash

echo "Building 'events-finder' docker image"
docker build -t events-finder .
DOCKER_BUILD_EXIT_CODE=$?
if [ $DOCKER_BUILD_EXIT_CODE -ne 0 ]; then
    echo "Docker build failed with exit code $DOCKER_BUILD_EXIT_CODE"
    exit $DOCKER_BUILD_EXIT_CODE
fi

docker-compose up -d
DOCKER_COMPOSE_EXIT_CODE=$?
if [ $DOCKER_COMPOSE_EXIT_CODE -ne 0 ]; then
    echo "Docker compose failed with exit code $DOCKER_COMPOSE_EXIT_CODE"
    exit $DOCKER_COMPOSE_EXIT_CODE
fi
