use std::env;

use fjage_rs::{
    api::gateway::Gateway,
    core::message::{Message, Performative},
};
use serde_json::json;

static HELP_STRING: &str = r##"
Usage: tx_datagram <hostname> <port> <remote node> <message>

transmit a datagram to the remote peer.
"##;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    // Validate arguments
    if args.len() < 5 {
        println!("{}", HELP_STRING);
        return;
    }
    let hostname: &str = args.get(1).unwrap();
    let port: Result<u16, _> = args.get(2).unwrap().parse();
    if port.is_err() {
        println!("Port must be integer!");
        println!("{}", HELP_STRING);
    }
    let port = port.unwrap();
    let dst: u32 = args.get(3).unwrap().parse().unwrap();
    let data = args.get(4).unwrap();

    // Connect to gateway
    let mut gw = Gateway::new_tcp(hostname, port);

    // Find an agent advertising the DATAGRAM service
    let dsp = gw
        .agent_for_service("org.arl.unet.Services.DATAGRAM")
        .unwrap();

    // Demonstration of the power of generic messages
    let datagram = Message::new_generic(
        "org.arl.unet.DatagramReq",
        Performative::REQUEST,
        json!({"data": data.as_bytes().to_vec(), "to": dst, "protocol": 0}),
    );

    // Send message to the datagram service provider
    let rsp = gw.request(&dsp, datagram);
    let rsp = rsp.unwrap();
    if match rsp.data.perf {
        Performative::AGREE => true,
        _ => false,
    } {
        println!("Transmitted successfully.");
    } else {
        println!("Transmission failed!");
    }
}
