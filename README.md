### Build docker image

```
./buildDocker.sh
```

## Run docker container

```
docker run -it -e ZMQ_SERVER_HOST=loclhost -e ZMQ_SOCKS_PROXY=bla zmq-rust-hello-client:latest

```
