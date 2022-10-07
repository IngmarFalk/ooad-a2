
# Setup

To be able to run this, you are going to have docker installed.
To install docker, pls refer to the official website:

Go here to [download docker](<https://docs.docker.com/get-docker/>)

# How to check out the project

## **Building**

To build the project run:

```docker
~ docker build -t a2 .
```

## **Running**

```docker
First - Enter the environment:
~ docker container run --rm -it `build_name` bash

You will then enter the container environment:
root@e7417beb0975:/usr/src/a2# ~

You can then run:
root@e7417beb0975:/usr/src/a2# ~ cargo run
```

## **Test**

```docker
First - Enter the environment:
~ docker container run --rm -it `build_name` bash

Then - in the container environment type:
root@e7417beb0975:/usr/src/a2# ~ cargo test
```

## **Linting**

```docker
First - Enter the environment:
~ docker container run --rm -it `build_name` bash

Then - in the container environment type:
root@e7417beb0975:/usr/src/a2# ~ cargo clippy --all
```
