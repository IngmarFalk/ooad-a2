
# Setup

*To be able to run this, you are going to have docker installed.
To install docker, please refer to the official website:*

**Go here to [download docker](<https://docs.docker.com/get-docker/>)**

# How to check out the project

## **Building**

**To build the project run:**

```docker
docker build -t a2 .
```

## **Running**

**First - Enter the environment:**

```bash
docker container run --rm -it `build_name` bash
```

**You will then enter the container environment:**

```bash
root@e7417beb0975:/usr/src/a2# ~
```

**You can then run:**

```bash
root@e7417beb0975:/usr/src/a2# ~ cargo run
```

## **Test**

**First - Enter the environment:**

```bash
docker container run --rm -it `build_name` bash
```

**Then - in the container environment type:**

```bash
root@e7417beb0975:/usr/src/a2# ~ cargo test
```

## **Linting**

**First - Enter the environment:**

```bash
docker container run --rm -it `build_name` bash
```

**Then - in the container environment type:**

```bash
root@e7417beb0975:/usr/src/a2# ~ cargo clippy --all
```

## The Test Report

The test report is an automatically generated table derived from the output of the `cargo test` command piped into the `test_results` file via the command: `cargo test > test_results`
