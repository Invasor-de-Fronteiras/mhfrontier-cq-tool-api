# MH Frontier CQ Toll Api

This API is for the web project version of [MHFrontier CQ Tool](https://github.com/Invasor-de-Fronteiras/mhfrontier-cq-tool). It serves as the core editor responsible for editing Monster Hunter Frontier quest binary files

### Table of Contents

- [Routes](#routes)
- [Local development](#local-development)
- [License](#license)

## Routes

### Read quest file request

```
POST /quest/read HTTP/1.1
Host: localhost:8080
Content-Type: multipart/form-data
```
**Form**

| Key    | Type | Value |
|---------|-------|--------|
| file    | File    | Binary quest file path |

### Edit quest file request

```
POST /quest/edit HTTP/1.1
Host: localhost:8080
Content-Type: multipart/form-data
```
**Form**

| Key    | Type | Value |
|---------|-------|--------|
| file    | File    | Binary quest file path |
| quest   | text    | JSON quest  |


## Local Development

### Requirements

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)

### Instructions

Run `cargo run` to start the project:

```sh
cargo run
```

To run as a docker container:
```sh
docker run ghcr.io/invasor-de-fronteiras/mhfrontier-cq-tool-api:latest
```

## Contributing

Feel the advantage for any contributions! If you have any questions, don't hesitate to join [our community]() server and ask as many questions as you like.

## License

Licensed under [MIT](/LICENSE).
