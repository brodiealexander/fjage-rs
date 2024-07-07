use std::env;

use fjage_rs::{
    core::message::{Message, Performative},
    remote::{
        file::{GetFileReq, GetFileRsp},
        gateway::Gateway,
    },
};
use serde_json::{json, Value};
use tokio::{
    fs::{self, File, OpenOptions},
    io::AsyncWriteExt,
};

static HELP_STRING: &str = r##"
Usage: rx_datagram <hostname> <port>

receive a datagram (interprets contents as string).
"##;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    // Validate arguments
    if args.len() < 3 {
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

    // Connect to gateway
    let mut gw = Gateway::new_tcp(hostname, port).await;

    // Find and subscribe to all agents advertising the DATAGRAM service
    let dsp = gw
        .agents_for_service("org.arl.unet.Services.DATAGRAM")
        .await;
    for agent in dsp.iter() {
        gw.subscribe_agent(&agent).await;
    }

    // Receive, filtering for DatagramNtf
    let rsp = gw
        .recv(Some(vec!["org.arl.unet.DatagramNtf".to_string()]), None)
        .await;
    let rsp = rsp.unwrap();

    if rsp.data.fields.contains_key("data") {
        // TODO: Make this API more ergonomic. Message::get_byte_array etc.
        let data = rsp.data.fields.get("data").unwrap().as_array().unwrap();
        // String from Vec<Value> where Value is of type u8.
        let data =
            String::from_utf8(data.iter().map(|x| x.as_u64().unwrap() as u8).collect()).unwrap();
        println!("Received: {}", data);
    } else {
        println!("Received blank message.");
    }
}
