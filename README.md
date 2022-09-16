
# Setup

To be able to run this, you are going to have docker installed.
To install docker, pls refer to the official website:

Go here to <a href=https://docs.docker.com/get-docker/>Download Docker</a>

## How to check out the project

### Building

To build the project run:

```docker
docker build -t `build_name` .
```

e.g.:

```docker
docker build -t assignment2 .
```

### Running

```docker
docker run `build_name`
```

e.g.:

```docker
docker run assignment2
```

### Linting

Linting is integrated into the compiler. Meaning when this project is run via `docker run ..` the compiler will display all
stylistic problems and errors. When there are no errors, warnings or tips, this means the code is clean and functioning.

