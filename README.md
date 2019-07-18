# multi-ipc-test

This test Rust project is an experiment utilizing
[Servo’s ipc-channel crate](https://github.com/servo/ipc-channel). It features
two programs, a server which is meant to be run once, and a client which can be
run multiple times. This architecture may be used to send tasks a service during
its runtime.

The server creates an [IpcOneShotServer](https://doc.servo.org/ipc_channel/ipc/struct.IpcOneShotServer.html)
instance and provides its name which is passed to the client as a command line
argument. The client reads standard input and sends the read data to the
specified server together with an
[IpcSender](https://doc.servo.org/ipc_channel/ipc/struct.IpcSender.html)
to provide a callback channel. As the server is one-shot, it is consumed upon
accepting the input; it is replaced with a different server, with a different
name (which is sent back to the client via the IpcSender). The next client
connects to that address, sends its input, receives the new server’s name
and so on.

This is an experiment and, as an implementation of the single server/multiple
client architecture, is quite crude and probably not optimal. Any suggestions of
alternative implementations are welcome.

## Usage

This instruction assumes you are using a Unix-like shell. Cargo’s diagnostic
messages are omitted. The server names may be different.

Run the `server` binary in a console or terminal, such as by using
`cargo run --bin server`. The server will print the first name to connect to:
```
$ cargo run --bin server
<...>
Server name is /tmp/.tmpZ3eONd/socket
```

Run the `client` binary in a second terminal or console. Specify the server name
printed by `server` as its command line argument. The `client` will read data
from standard input until reaching end of line; you can pipe the output of any
program or contents of any file. The client will send the message and receive
the new server’s address:
```
$ echo 'Hello, world!' | cargo run --bin client /tmp/.tmpZ3eONd/socket
<...>
Packet successfully sent to /tmp/.tmpZ3eONd/socket
New server name: /tmp/.tmpS9E61v/socket
```
The server will print `Hello, world!`.

Each new execution of `client` must use the server name provided by the previous
client:
```
$ echo 'Hello fellow Rustaceans!' | cargo run --bin client /tmp/.tmpS9E61v/socket
<...>
Packet successfully sent to /tmp/.tmpS9E61v/socket
New server name: /tmp/.tmpPBlkPy/socket
```
Attempts to use an old address will very likely result in a panic.

To shut down the server, pass `quit` as the second argument. The client will
still read data from standard input:
```
$ echo | cargo run --bin client /tmp/.tmpPBlkPy/socket stop
<...>
Packet successfully sent to /tmp/.tmpPBlkPy/socket
Server is shutting down. Goodbye fellow Rustaceans!
```

## License

As an experiment not intended for practical use, this project is placed into
public domain according to terms of
[the Unlicense](http://unlicense.org).