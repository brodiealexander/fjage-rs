//use std::sync::mpsc;

use tokio::{
    io::{self, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
    runtime::{Handle, Runtime},
    sync::mpsc::{self, Receiver, Sender, UnboundedReceiver, UnboundedSender},
};

use super::frame::Frame;

pub trait Connector {
    async fn connect(&self) -> (UnboundedSender<Frame>, UnboundedReceiver<Frame>);
}

pub struct TcpConnector {
    hostname: String,
    port: u16,
}

impl TcpConnector {
    pub fn new(hostname: &str, port: u16) -> TcpConnector {
        TcpConnector {
            hostname: String::from(hostname),
            port: port,
        }
    }
    async fn read_task<T: AsyncRead + Unpin>(stream: T, mut sender: UnboundedSender<Frame>) {
        let mut reader = BufReader::new(stream);
        loop {
            let mut line_in = String::new();
            let bytes_read = reader.read_line(&mut line_in).await;
            if (bytes_read.is_err()) {
                panic!("Error: TcpConnector could not read: {:?}", bytes_read.err());
            }
            let bytes_read = bytes_read.unwrap();
            if (bytes_read == 0) {
                panic!("Error: TcpConnector reached EOF!");
            }
            let frame = Frame::from_json(&line_in);
            if frame.is_some() {
                let frame = frame.unwrap();
                println!("\nTcpConnector << Remote: {:?}", frame);
                sender.send(frame).unwrap();
            } else {
                println!("Error: TcpConnector could not parse frame: {:?}", line_in);
            }
        }
    }
    async fn write_task<T: AsyncWrite + Unpin>(stream: T, mut receiver: UnboundedReceiver<Frame>) {
        let mut writer = BufWriter::new(stream);
        loop {
            let frame = receiver.recv().await;
            if frame.is_none() {
                println!("Error: Receive channel closed for TcpConnector!");
                continue;
            }
            let mut frame = frame.unwrap();
            println!("\nTcpConnector >> Remote: {:?}", frame);
            writer.write(frame.to_json().as_bytes()).await.unwrap();
            writer.write(b"\n").await.unwrap();
            writer.flush().await.unwrap();
            //let bytes_read = reader.read_line(&mut line_in);
        }
    }
}
impl Connector for TcpConnector {
    async fn connect(&self) -> (UnboundedSender<Frame>, UnboundedReceiver<Frame>) {
        //let handle = Handle::current();
        let input_stream = TcpStream::connect(format!("{}:{}", self.hostname, self.port))
            .await
            .unwrap();

        let (rstream, wstream) = io::split(input_stream);

        let (client_to_conn, conn_from_client): (UnboundedSender<Frame>, UnboundedReceiver<Frame>) =
            mpsc::unbounded_channel();
        let (conn_to_client, client_from_conn): (UnboundedSender<Frame>, UnboundedReceiver<Frame>) =
            mpsc::unbounded_channel();

        tokio::spawn(async move {
            TcpConnector::read_task(rstream, conn_to_client).await;
        });
        tokio::spawn(async move {
            TcpConnector::write_task(wstream, conn_from_client).await;
        });

        return (client_to_conn, client_from_conn);
    }
}