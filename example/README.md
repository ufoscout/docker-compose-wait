## Example with docker image

You can have an example with `compose` in [docker-compose.yml](./docker-compose.yml).

To try this example use the following commands:

```sh
$ docker-compose build
$ docker-compose up -d postgres && docker-compose up wait_postgres
```