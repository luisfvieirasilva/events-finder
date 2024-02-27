#!/bin/bash

echo "Exporting 'events-finder' KeyCloak realm"
docker-compose exec keycloak /opt/keycloak/bin/kc.sh export --file /opt/keycloak/data/events_finder-realm.json --realm events_finder --users skip
DOCKER_EXEC_EXIT_CODE=$?
if [ $DOCKER_EXEC_EXIT_CODE -ne 0 ]; then
    echo "Fail to export 'events-finder' realm" >&2
    exit $DOCKER_EXEC_EXIT_CODE
fi

echo "Copying 'events-finder' realm export from docker image"
docker-compose cp keycloak:/opt/keycloak/data/events_finder-realm.json events_finder_realm.json
DOCKER_CP_EXIT_CODE=$?
if [ $DOCKER_CP_EXIT_CODE -ne 0 ]; then
    echo "Fail to copy 'events-finder' realm export from docker image" >&2
    exit $DOCKER_CP_EXIT_CODE
fi

echo "Removing 'events-finder' realm export from docker image"
docker-compose exec keycloak rm /opt/keycloak/data/events_finder-realm.json
DOCKER_EXEC_EXIT_CODE=$?
if [ $DOCKER_EXEC_EXIT_CODE -ne 0 ]; then
    echo "Fail to remove 'events-finder' realm export from docker image" >&2
    exit $DOCKER_EXEC_EXIT_CODE
fi
