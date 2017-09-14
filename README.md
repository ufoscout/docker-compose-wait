# docker-entrypoint

An entrypoint script that can be used to wait for services to be started and or populate environment variables from a file.

## Waiting For a Service to Start

Wait for external services to be started before starting the Docker container. The script will wait for a fixed amount of seconds until a TCP port is open on a service.

Example:

```
$ docker run --name some-mysql-client \
  -e MYSQL_PASSWORD=password \
  -e WAIT_HOSTS=mysql:3306 \
  -d mysql-client:tag
```

## Docker Secrets

As an alternative to passing information via environment variables, _FILE may be appended to an environment variable, entrypoint.sh will load the values for those variables from files present in the container. In particular, this can be used to load passwords from Docker secrets stored in /run/secrets/<secret_name> files.

Example:

```
$ docker run --name some-mysql-client \
  -e MYSQL_ROOT_PASSWORD_FILE=/run/secrets/mysql-root \
  -d mysql-client:tag
```

# Usage
The script is used in the docker build process and setup as the ENTRYPOINT for the container.

Modify the Dockerfile file:
## Dockerfile
```
FROM ubuntu

## Add your application to the docker image
ADD run.sh /run.sh
RUN chmod +x /run.sh

## Add the entrypoint.sh script to the image
ADD https://raw.githubusercontent.com/c7ks7s/docker-entrypoint/master/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

## Install netcat - required for WAIT_HOSTS in entrypoint.sh
RUN apt-get update && \
    apt-get install -y netcat && \
    apt-get clean && \
    apt-get purge && \
    rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/entrypoint.sh"]
CMD ["/run.sh"]
```

Modify the docker-compose.yml file to consume secrets and wait for the mysql database:
## docker-compose.yml
```
version: "2"

secrets:
  mysql_password:
    file: ./secrets/mysql_password
services:
  mysql:
    image: "mysql:5.7"
    ports:
      - "3306:3306"
  myApp:
    image: "myApp:latest"
    secrets:
      - mysql_password:
    environment:
      WAIT_HOSTS: mysql:3306
      MYSQL_PASSWORD_FILE=/run/secrets/mysql_password
```

docker-compose will start 'myApp' when all the pairs host:port in the WAIT_HOSTS variable are available and the environment variable MYSQL_PASSWORD will be populated from the docker secrets.

## Configuration Options:

The WAIT_HOSTS and *_FILE environment variables are not mandatory, if not declared, entrypoint.sh executes the Docker Command.

entrypoint.sh can be configured with the following environment variables:
- WAIT_HOSTS: comma separated list of pairs host:port for which the script will wait
- WAIT_HOSTS_TIMEOUT: max number of seconds to wait the hosts to be available before failure. The default is 30 seconds.
- WAIT_BEFORE_HOSTS: number of seconds to wait (sleep) before start checking for the hosts availability
- WAIT_AFTER_HOSTS: number of seconds to wait (sleep) once all the hosts are available
- <ENV_VAR>_FILE: populate <ENV_VAR> from the file
