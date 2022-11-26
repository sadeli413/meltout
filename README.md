# Meltout

Meltout is a command and control framework. This is basically a toy in developement.

There is no authentication in this thing yet so don't actually use this.

## Features
- Meltout supports (optional) multiplayer.
- The Operator-Server and Server-Implant communication is done via gRPC over HTTP/2
- Meltout only works in beacon mode, for executing shell commands.

## Running Meltout For Developement
1. Build and run the Docker container
```
$ docker build . -t meltout
$ docker run --rm --name server --hostname server -it meltout
```

2. Run the server
```
meltout:~$ cargo run --bin meltout-server
```

3. Start listening for implants
```
>> listeners new --lhost 172.17.0.2 --server-pem certs/server.pem --server-key certs/server.key
```

## Compiling the Implant
```
meltout:~$ cd implant
meltout:~/implant$ MELTOUT_CERT_FOLDER=../certs/ MELTOUT_DOMAIN_NAME='meltout.server' MELTOUT_ENDPOINT='https://172.17.0.2:9001' cargo build --release
```

The binary should be located in `~/implant/target/release/implant`.
