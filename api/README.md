# Passport SpaghettETH API

## Install dependencies and run server

```
rustup default stable
bash scripts/run
```

## Create .env file

```
API_VERSION=v1
MONGODB_CONNECTION=mongodb://localhost:27017
MONGODB_DB=passport
```

## Run dev mode

```
cargo install cargo-watch
cargo dev
```

## Use with docker

```
docker build -t passport-api-rs .
bash scripts/docker/run
```