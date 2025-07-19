use core::num;
use std::{
    env,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::anyhow;
use clap::Parser;
use fjage_rs::{api::gateway::Gateway, core::message::Performative, remote::file::PutFileReq};
use indicatif::{ProgressBar, ProgressStyle};

use std::fs::OpenOptions;

/// Built to bulk upload signals according to MuNet @ University of Alabama's signals.yaml format
/// [SIGNAL_LABEL]: "path/to/signal.txt"
/// e.g. MY_SIGNAL_NAME_ON_MODEL: "my/cool/signal.txt"
#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t=String::from("signals.yaml"))]
    signals_file: String,
    #[arg(long, default_value_t=String::from("192.168.0.127"))]
    hostname: String,
    #[arg(long, default_value_t = 1100)]
    port: u16,
    #[arg(long, default_value_t = 1024*64)]
    chunk_len: u32,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let path = Path::new(&args.signals_file);
    if !path.exists() {
        println!("{} not found!", args.signals_file);
    }

    // Read the config file into memory
    let file = OpenOptions::new().read(true).open(path)?;
    let config: serde_yml::Mapping = serde_yml::from_reader(file)?;
    //let mut contents = String::new();
    //file.read_to_string(&mut contents).unwrap();

    // Connect to gateway
    let mut gw = Gateway::new_tcp(&args.hostname, args.port);

    // Find an agent advertising the SHELL service
    let shell = gw
        .agent_for_service("org.arl.fjage.shell.Services.SHELL")
        .unwrap();

    // Subscribe to the shell agent
    gw.subscribe_agent(&shell);

    let chunk_len: u64 = args.chunk_len as u64;

    for (label, local_path) in config {
        let Some(label) = label.as_str() else {
            return Err(anyhow!(format!(
                "Label to string conversion failed: {label:?}"
            )));
        };
        let Some(local_path) = local_path.as_str() else {
            return Err(anyhow!(format!(
                "Path to string conversion failed: {local_path:?}"
            )));
        };
        let path = Path::new(&local_path);
        if !path.exists() {
            return Err(anyhow!(format!(
                "Local path does not exist: {local_path:?}"
            )));
        }
        let local_file = OpenOptions::new().read(true).open(local_path)?;
        let local_file_size = local_file.metadata()?.len();
        let num_chunks = local_file_size.div_ceil(args.chunk_len as u64);
        let mut reader = BufReader::new(local_file);
        let mut buffer = vec![0_u8; chunk_len as usize];
        let mut offset: u64 = 0;

        let remote_file_path = format!("/home/unet/scripts/{}.txt", label);

        let bar = ProgressBar::new(num_chunks);
        bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] |{bar:40.cyan/blue}| {pos:>7}/{len:7} {msg}",
            )
            .unwrap(),
        );

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
                //println!("FILE CHUNK UPLOADED SUCCESSFULLY");
            } else {
                println!("FILE CHUNK FAILED TO UPLOAD");
            }

            //bytes_read = reader.read(&mut buffer).unwrap();
            offset = offset + bytes_read as u64;
            //println!("Bytes read: {bytes_read}, offset: {offset}");
            bar.inc(1);
            bar.set_message(format!(
                "[{label}: {}M/{}M ({}%)]",
                offset as f32 / 1_000_000.0,
                local_file_size as f32 / 1_000_000.0,
                offset as f32 / local_file_size as f32 * 100.0
            ));
        }
        bar.finish();
    }

    println!("FINISHED");

    Ok(())
}
