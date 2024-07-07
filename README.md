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
