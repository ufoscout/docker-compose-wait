#!/usr/bin/env bash

echo "google should be immediately found and the Docker CMD should echo 'success'"
echo 
WAIT_HOSTS=google.com:80 ./entrypoint.sh echo 'success' | sed -e 's/^/    /g'

echo

echo "nonexistent server should error out without executing the docker CMD"
echo 
WAIT_HOSTS=noserver:9999 WAIT_HOSTS_TIMEOUT=5 ./entrypoint.sh echo 'success' | sed -e 's/^/    /g'

echo

echo "No such file used with ENV_VAR_FILE and the docker CMD should echo 'success'"
echo 
ENV_VAR_FILE=./no_such_file ./entrypoint.sh echo 'success' | sed -e 's/^/    /g'

echo

echo "populate ENV_VAR from ./secret_file and the docker CMD should echo 'success'"
echo 
echo 'Hello World' > secret_file
ENV_VAR_FILE=./secret_file ./entrypoint.sh echo 'success' | sed -e 's/^/    /g'
rm secret_file
