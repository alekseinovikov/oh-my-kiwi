use crate::core::config::TcpConfig;
use crate::parser::parser::KiwiCommandParser;
use crate::parser::reader::TcpBufferedReader;
use crate::processor::processor::KiwiCommandProcessor;
use crate::processor::writer::{KiwiResponseWriter, TcpBytesWriter};
use crate::server::server::RESP3Server;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::{error, info};

pub(crate) struct TcpServer {
    tcp_config: TcpConfig,
}

impl TcpServer {
    pub(crate) fn new(tcp_config: TcpConfig) -> TcpServer {
        Self { tcp_config }
    }

    pub(crate) async fn run(&self) -> std::io::Result<()> {
        let socket_addr = self.tcp_config.socket_addr()?;
        let listener = TcpListener::bind(socket_addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;
            info!("New connection from: ${addr}");

            tokio::spawn(async move {
                if let Err(e) = Self::handle_client(stream).await {
                    error!("Error with {}: {:?}", addr, e);
                }
            });
        }
    }

    async fn handle_client(stream: TcpStream) -> std::io::Result<()> {
        let processor = KiwiCommandProcessor::new();
        let stream = Arc::new(Mutex::new(stream));

        let bytes_reader = TcpBufferedReader::new(stream.clone());
        let bytes_writer = TcpBytesWriter::new(stream.clone());

        let parser = KiwiCommandParser::new(bytes_reader);
        let writer = KiwiResponseWriter::new(bytes_writer);

        let mut server = RESP3Server::new(parser, processor, writer);
        server.run().await;
        Ok(())
    }
}
