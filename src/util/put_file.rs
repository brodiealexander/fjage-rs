use std::{
    env,
    io::{BufReader, Read},
    path::Path,
};

use fjage_rs::{api::gateway::Gateway, core::message::Performative, remote::file::PutFileReq};

use std::fs::OpenOptions;

static HELP_STRING: &str = r##"
Usage: put_file <hostname> <port> <local file> <remote file>

If no local file is specified, the output will be printed to the console.
"##;
fn main() {
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
    let mut file = OpenOptions::new().read(true).open(&path).unwrap();
    //let mut contents = String::new();
    //file.read_to_string(&mut contents).unwrap();

    // Connect to gateway
    let mut gw = Gateway::new_tcp(hostname, port);

    // Find an agent advertising the SHELL service
    let shell = gw
        .agent_for_service("org.arl.fjage.shell.Services.SHELL")
        .unwrap();

    // Subscribe to the shell agent
    gw.subscribe_agent(&shell);

    let chunk_len: u64 = 1024 * 64;

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::<u8>::with_capacity(chunk_len as usize);
    buffer.resize(chunk_len as usize, 0);
    let mut offset: u64 = 0;

    while let Ok(bytes_read) = reader.read(&mut buffer[0..chunk_len as usize]) {
        if bytes_read == 0 {
            break;
        }
        let mut msg = PutFileReq::new_contents(&remote_file_path, &buffer[0..bytes_read]);
        msg.ofs = offset;
        let rsp = gw.request(&shell, msg.to_msg());
        let rsp = rsp.unwrap();
        if match rsp.data.perf {
            Performative::AGREE => true,
            _ => false,
        } {
            println!("FILE CHUNK UPLOADED SUCCESSFULLY");
        } else {
            println!("FILE CHUNK FAILED TO UPLOAD");
            panic!("Uh oh!");
        }

        //bytes_read = reader.read(&mut buffer).unwrap();
        offset = offset + bytes_read as u64;
        println!("Bytes read: {bytes_read}, offset: {offset}");
    }

    println!("FINISHED");

    // Construct and send a PutFileReq
}
