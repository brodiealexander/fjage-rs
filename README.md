# fjage-rs
fjage-rs is an experimental rust gateway for [fjåge](https://github.com/org-arl/fjage/tree/master). It is not ready for use, but passes the fjåge C gateway API tests. 

A general description of what exists so far:

- protocol/base64.rs: helper methods for base64 encoding and decoding
- protocol/connector.rs: implementation of the Connector concept from fjåge. Only TcpConnectors are implemented.
- protocol/frame.rs: implementation of JsonMessage from the [fjåge protocol docs](https://fjage.readthedocs.io/en/latest/protocol.html).
- core/message.rs: implementation of the 'message' field of JsonMessage
- core/param.rs: implementation of ParameterReq and ParameterRsp as well as setters and getters
- remote/gateway.rs: main implementation of the gateway. Resembles a container.
- ffi/ : implementation of the C API compatibility layer

# Getting Started / Running the fjåge C gateway tests

To run the fjåge C gateway tests:
- run `cargo build` in the project root
- navigate to `tests/docker` and run `./launch-arm64.sh` or `./launch-amd64.sh`. 
- once you see the fjåge shell prompt, type `run 'dummy'` to load the test fixture agent.
- in another terminal, navigate to `/tests/fjage-c` and run `make fjage-rs-test`. Then run `./test_fjage` with a container running.

# Viewing docs

Run `cargo doc --open` to view documentation for this package in your browser. 