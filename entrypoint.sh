#!/bin/bash

set -eo pipefail
shopt -s nullglob

#
# populates an environment variable from a file useful with docker secrets
#
secretDebug()
{
  if [ ! -z "$ENV_SECRETS_DEBUG" ]; then
    echo -e "\033[1m$@\033[0m"
    echo
  fi
}

getSecrets () {
  for env_var in $(printenv | cut -f1 -d"=" | grep _FILE)
  do
    name="$env_var"
    eval value=\$$name

    if [ -f "$value" ]; then
      value=$(cat "${value}")
      export "${name%_FILE}"="$value"
      unset $name
      secretDebug "Expanded Secret! ${name%_FILE}=$value"
    else
      secretDebug "Secret file does not exist! $value"
    fi
  done
}

ENV_SECRETS_DEBUG=1
getSecrets

#
# wait for a service to start
#

timeout=${WAIT_HOSTS_TIMEOUT:-30}
waitAfterHosts=${WAIT_AFTER_HOSTS:-0}
waitBeforeHosts=${WAIT_BEFORE_HOSTS:-0}

if [ $waitBeforeHosts != 0 ];
then
  echo "Waiting for ${waitBeforeHosts} seconds."
  sleep $waitBeforeHosts
fi

# target format is a comma separated list where each item is "host:ip"
if [ -n "$WAIT_HOSTS" ]; then
  uris=$(echo $WAIT_HOSTS | sed -e 's/,/ /g' -e 's/\s+/\n/g' | uniq)
fi

# wait for each target
if [ ! -z "$uris" ];
then
  for uri in $uris
  do
    host=$(echo $uri | cut -d: -f1)
    port=$(echo $uri | cut -d: -f2)
    [ -n "${host}" ]
    [ -n "${port}" ]
    echo "Waiting for ${uri}."
    seconds=0
    while [ "$seconds" -lt "$timeout" ] && ! timeout --preserve-status 1 nc -z -w1 $host $port 2>/dev/null
    do
      echo -n .
      seconds=$((seconds+1))
      sleep 1
    done

    if [ "$seconds" -lt "$timeout" ];
    then
      echo "${uri} is up!"
    else
      echo
      echo "ERROR: unable to connect to ${uri}" 
      exit 1
    fi
  done
  echo "All hosts are up"
  echo
fi

if [ $waitAfterHosts != 0 ];
then
  echo "Waiting for ${waitAfterHosts} seconds."
  sleep $waitAfterHosts
fi

#
# End
#

echo "Executing Docker CMD"
exec "$@"
