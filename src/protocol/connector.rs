//use std::sync::mpsc;

use tokio::{
    io::{self, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
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
                //println!("\nTcpConnector << Remote: {:?}", frame);
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
            //println!("\nTcpConnector >> Remote: {:?}", frame);
            writer.write_all(frame.to_json().as_bytes()).await.unwrap();
            writer.write_all(b"\n").await.unwrap();
            writer.flush().await.unwrap();
        }
    }
}
impl Connector for TcpConnector {
    async fn connect(&self) -> (UnboundedSender<Frame>, UnboundedReceiver<Frame>) {
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

pub struct SerialPortConnector {
    dev: String,
    baud: u32,
}

impl SerialPortConnector {
    pub fn new(dev: &str, baud: u32) -> SerialPortConnector {
        SerialPortConnector {
            dev: String::from(dev),
            baud: baud,
        }
    }
    async fn read_task<T: AsyncRead + Unpin>(stream: T, mut sender: UnboundedSender<Frame>) {
        let mut reader = BufReader::new(stream);
        loop {
            let mut line_in = String::new();
            let bytes_read = reader.read_line(&mut line_in).await;
            if (bytes_read.is_err()) {
                panic!(
                    "Error: SerialPortConnector could not read: {:?}",
                    bytes_read.err()
                );
            }
            let bytes_read = bytes_read.unwrap();
            if (bytes_read == 0) {
                panic!("Error: SerialPortConnector reached EOF!");
            }
            let frame = Frame::from_json(&line_in);
            if frame.is_some() {
                let frame = frame.unwrap();
                //println!("\nTcpConnector << Remote: {:?}", frame);
                sender.send(frame).unwrap();
            } else {
                println!(
                    "Error: SerialPortConnector could not parse frame: {:?}",
                    line_in
                );
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
            //println!("\nTcpConnector >> Remote: {:?}", frame);
            writer.write_all(frame.to_json().as_bytes()).await.unwrap();
            writer.write_all(b"\n").await.unwrap();
            writer.flush().await.unwrap();
        }
    }
}
impl Connector for SerialPortConnector {
    async fn connect(&self) -> (UnboundedSender<Frame>, UnboundedReceiver<Frame>) {
        let sp = tokio_serial::new(self.dev.clone(), self.baud);
        let sp = tokio_serial::SerialStream::open(&sp).unwrap();

        let (rstream, wstream) = io::split(sp);

        let (client_to_conn, conn_from_client): (UnboundedSender<Frame>, UnboundedReceiver<Frame>) =
            mpsc::unbounded_channel();
        let (conn_to_client, client_from_conn): (UnboundedSender<Frame>, UnboundedReceiver<Frame>) =
            mpsc::unbounded_channel();

        tokio::spawn(async move {
            SerialPortConnector::read_task(rstream, conn_to_client).await;
        });
        tokio::spawn(async move {
            SerialPortConnector::write_task(wstream, conn_from_client).await;
        });

        return (client_to_conn, client_from_conn);
    }
}
