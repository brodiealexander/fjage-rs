use std::env;

use fjage_rs::{api::gateway::Gateway, core::message::Performative, remote::shell::ShellExecReq};

// Execute a shell command remotely

static HELP_STRING: &str = r##"
Usage: remote_shell_exec <hostname> <port> <command>
"##;

fn main() {
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
    let cmd: &str = args.get(3).unwrap();

    // Connect to gateway
    let mut gw = Gateway::new_tcp(hostname, port);
    // Find an agent advertising the SHELL service
    let shell = gw
        .agent_for_service("org.arl.fjage.shell.Services.SHELL")
        .unwrap();

    // Subscribe to the shell agent
    gw.subscribe_agent(&shell);
    let mut msg = ShellExecReq::new(cmd);
    let rsp = gw.request(&shell, msg.to_msg()).unwrap();

    if match rsp.data.perf {
        Performative::AGREE => true,
        _ => false,
    } {
        println!(
            "SUCCESS:\n{}",
            rsp.data.fields.get("ans").unwrap().as_str().unwrap()
        );
    } else {
        println!("COMMAND FAILED");
    }
}
