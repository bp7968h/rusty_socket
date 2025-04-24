# Web Socket Implementation in Rust
This is a learning attemp to implement web socket from scratch, including all the dependencies, which are implemented as a different library. With this `RFC 6455: The WebSocket Protocol` is implemented, including core protocol, server and client.

[![Rusty Socket Demo](banner.png)](rusty_socket_example.mp4)

## Overview

WebSockets provide a full-duplex communication channel over a single TCP connection. This implementation handles establishing connections, exchanging messages, and managing different frame types as specified in the WebSocket protocol.
Features

- `Protocol Implementation`: Comprehensive implementation of the WebSocket protocol, handling frames, opcodes, handshakes, and error management.
- `Server Implementation`: An extendable WebSocket server that accepts connections, manages clients, and broadcasts messages.
- `Client Implementation`: A simple client that connects to the WebSocket server and can send and receive messages.

## Core Libraries

1. `rusty_socket_core`: Contains the core WebSocket protocol logic including frame parsing, opcodes, and utilities.
2. `rusty_socket_client`: Implements the client-side functionalities, including establishing connections, sending messages, and handling responses.
3. `rusty_socket_server`: Implements the server-side functionalities, including accepting client connections, broadcasting messages, and managing the server state.

## Getting Started

### Prerequisites

Make sure you have Rust installed. You can install Rust by following the instructions from the [official Rust website](https://www.rust-lang.org/learn/get-started)..

### Building the Project
To build the entire project, run:
```bash
cargo build --release
```

### Running the Example Server
To start the example server, navigate to the `example/server` directory and run:
```bash
cargo run
```
The server will start listening on 127.0.0.1:8080. It will handle WebSocket handshake requests and maintain active connections with clients.

### Running the Example Client
To run the client, navigate to the `example/client` directory and run:
```bash
cargo run
```
The client can be used to connect to the example server and send or receive messages. Use commands like `connect`, `__disconnect`, `help`, and `exit` to control the client behavior.

## Protocol Implementation Details

- `Handshake`: The client initiates a WebSocket connection by sending an HTTP-based handshake request. The server responds with an appropriate handshake response if the connection is valid, upgrading the communication to WebSockets.
- `Frames`: The protocol supports different opcodes such as Text, Binary, Ping, Pong, and Close. Frames are parsed and processed following the WebSocket protocol specifications.
- `Error Handling`: Errors are managed with clear error messages, making it easier to debug issues during communication.

## Example Usage

Here’s a quick example of how to use the client to connect to the server:
```bash
---<Command Line Chat Initialized!>---
> help
Usage:
	connect: Connect to the server
	__disconnect: Disconnect from the server
	exit: Exit the program
	help: Prints usage guide
> connect
Connected successfully.
msg> Hello from client 1
rcv=> Hello from client 1
msg> rcv=> Hello from client 2
msg> __disconnect
Disconnected successfully!
> exit
Exiting Program!
```

## Todo
- Support frame fragmentation support.
- Enhance error handling and logging.
- Implement SSL/TLS support for wss:// connections.
- Add more examples and comprehensive documentation.
