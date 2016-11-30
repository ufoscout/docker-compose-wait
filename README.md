# docker-compose-wait
A simple script to wait for other docker images to be started while using docker-compose.
The script permits to wait for a fixed amount of seconds and/or to wait until a TCP port is open on a target image.

# Usage
The scripts must be used in docker build process and launched before your application starts.

For example, suppose that your application "MySuperApp" uses MongoDB, Postgres and MySql (wow!) and you wnat to be sure that when it starts all other systems are available, then you can customize your dockerfile this way:

```
FROM ubuntu

## Add your application to the docker image
ADD MySuperApp.sh /MySuperApp.sh

## Add the wait script to the image
ADD https://raw.githubusercontent.com/ufoscout/docker-compose-wait/master/wait.sh /wait.sh
RUN chmod +x /wait.sh

## Start the wait.sh script and then your application
CMD /wait.sh && /MySuperApp.sh
```

now your image is ready.

Your docker-compose.yml file will look like:

```
version: "2"

services:

  mongo:
    image: mongo:3.4
    hostname: mongo
  
  postgres:
    image: "postgres:9.4"
    hostname: postgres
    ports:
      - "5432:5432"

  mysql:
    image: "mysql:5.7"
    hostname: mysql
    ports:
      - "3306:3306"
      
  mySuperApp:
    image: "mySuperApp:latest"
    hostname: mySuperApp
    environment:
      WAIT_HOSTS: postgres:5432, mysql:3306, mongo:27017
```

Now when you start docker-compose, your application will be started only when all the pairs host:port in the WAIT_HOSTS variable are available.
The WAIT_HOSTS environment variable is not mandatory, if not declared, the script will execute without waiting.

## More configuration options
The behaviour of the wait.sh script can be configured with the following environment variables:
- WAIT_HOSTS: comma separated list of pairs host:port for which the script will wait
- WAIT_HOSTS_TIMEOUT: max number of seconds to wait the hosts to be available before failure. The default is 30 seconds.
- WAIT_BEFORE_HOSTS: number of seconds to wait (sleep) before start checking the hosts availability
- WAIT_AFTER_HOSTS: number of seconds to wait (sleep) once all the hosts are available


