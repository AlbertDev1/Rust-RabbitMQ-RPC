# Rust RabbitMQ RPC Example

This repository demonstrates how to implement a simple RPC (Remote Procedure Call) pattern using Rust and RabbitMQ. It consists of a client that sends a request to a server and waits for a response. The server processes the request and sends back a reply. This example aims to provide a practical introduction to using Rust with RabbitMQ for RPC communications.

## Prerequisites

Before running this project, you'll need:

- Rust installed on your machine. [Installation guide](https://www.rust-lang.org/tools/install)
- RabbitMQ server running locally or remotely. [Installation guide](https://www.rabbitmq.com/download.html)

## Setup

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/AlbertDev1/Rust-RabbitMQ-RPC.git
   cd Rust-RabbitMQ-RPC


## Running the Example

To get the RPC interaction between the client and server up and running, follow these steps:

### Start the Server

Open a terminal and run the server with the following command:

```bash
cargo run --bin server

cargo run --bin client