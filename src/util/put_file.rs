use std::{env, path::Path};

use fjage_rs::{
    core::message::{Message, Performative},
    remote::{
        file::{GetFileReq, GetFileRsp, PutFileReq},
        gateway::Gateway,
    },
};
use serde_json::Value;
use tokio::{
    fs::{self, File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

static HELP_STRING: &str = r##"
Usage: put_file <hostname> <port> <local file> <remote file>

If no local file is specified, the output will be printed to the console.
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
        return;
    }
    let port = port.unwrap();

    let local_file_path = args.get(3).unwrap();
    let remote_file_path = args.get(4).unwrap();

    let path = Path::new(local_file_path);
    if !path.exists() {
        println!("PATH NOT FOUND");
    }

    // Read the file into memory
    let mut file = OpenOptions::new().read(true).open(&path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();

    // Connect to gateway
    let mut gw = Gateway::new_tcp(hostname, port).await;

    // Find an agent advertising the SHELL service
    let shell = gw
        .agent_for_service("org.arl.fjage.shell.Services.SHELL")
        .await
        .unwrap();

    // Subscribe to the shell agent
    gw.subscribe_agent(&shell).await;

    // Construct and send a PutFileReq
    let mut msg = PutFileReq::new_contents(&remote_file_path, &contents);

    let rsp = gw.request(&shell, msg.to_msg()).await;
    let rsp = rsp.unwrap();
    if match rsp.data.perf {
        Performative::AGREE => true,
        _ => false,
    } {
        println!("FILE UPLOADED SUCCESSFULLY");
    } else {
        println!("FILE FAILED TO UPLOAD");
    }
}