## Document-Vader

Sample client-server program in rust similar to ftp made with rust but with EC and emojis

- uses gRPC
- Reed Solomon for EC files recovery

```
rustc 1.69.0 (84c898d65 2023-04-16)
cargo 1.69.0 (6e9a83356 2023-04-12)
```

### Pre-reqs

```shell
# needed for gRPC
sudo apt install -y cmake
```

### Server

```bash
cd blob_server
cargo run
```

### Client

```shell
cd cli_client
cargo run
```
Happy Path Commands
```shell
connect http://0.0.0.0:50051
put /some/file.txt
status
get /some/file.txt /some/cp-file.txt
exit
```