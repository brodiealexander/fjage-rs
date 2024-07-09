use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use fjage_rs::{
    api::gateway::Gateway,
    remote::file::{GetFileReq, GetFileRsp},
};

static HELP_STRING: &str = r##"
Usage: get_file <hostname> <port> <remote file/dir> [local file]

If no local file is specified, the output will be printed to the console.
"##;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Validate arguments
    if args.len() < 4 {
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
    let mut gw = Gateway::new_tcp(hostname, port);

    // Find an agent advertising the SHELL service
    let shell = gw
        .agent_for_service("org.arl.fjage.shell.Services.SHELL")
        .unwrap();

    // Subscribe to the shell agent
    gw.subscribe_agent(&shell);

    // Construct and send a GetFileReq
    let mut msg = GetFileReq::new(args.get(3).unwrap());
    let rsp = gw.request(&shell, msg.to_msg());
    let rsp = GetFileRsp::from_msg(rsp.unwrap());

    // If the user specified an output file, write response to it. If not, print output to the console
    if args.len() == 5 {
        let path = Path::new(args.get(4).unwrap());
        let parent = path.parent();
        if parent.is_some_and(|x| !x.exists()) {
            fs::create_dir_all(parent.unwrap()).unwrap();
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        file.write_all(&rsp.contents).unwrap();
    } else {
        println!(
            "File Contents:\n{}",
            std::str::from_utf8(&rsp.contents).unwrap()
        );
    }
}
