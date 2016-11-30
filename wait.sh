#!/bin/bash

set -e

timeout=${WAIT_HOSTS_TIMEOUT:-30}
waitAfterHosts=${WAIT_AFTER_HOSTS:-0}
waitBeforeHosts=${WAIT_BEFORE_HOSTS:-0}
 
echo "Waiting for ${waitBeforeHosts} seconds."
sleep $waitBeforeHosts

# our target format is a comma separated list where each item is "host:ip"
if [ -n "$WAIT_HOSTS" ]; then 
  uris=$(echo $WAIT_HOSTS | sed -e 's/,/ /g' -e 's/\s+/\n/g' | uniq)
fi

# wait for each target
if [ -z "$uris" ];
  then echo "No wait targets found." >&2;

  else 

  for uri in $uris
  do
    host=$(echo $uri | cut -d: -f1)
    port=$(echo $uri | cut -d: -f2)
    [ -n "${host}" ]
    [ -n "${port}" ]
    echo "Waiting for ${uri}."
    seconds=0
    while [ "$seconds" -lt "$timeout" ] && ! nc -z -w1 $host $port
    do
      echo -n .
      seconds=$((seconds+1))
      sleep 1
    done

    if [ "$seconds" -lt "$timeout" ]; then
      echo "${uri} is up!"
    else
      echo "  ERROR: unable to connect to ${uri}" >&2
      exit 1
    fi
  done
echo "All hosts are up"
fi

echo "Waiting for ${waitAfterHosts} seconds."
sleep $waitAfterHosts

exit 0 
