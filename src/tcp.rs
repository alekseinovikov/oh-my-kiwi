use crate::config::TcpConfig;
use crate::parser::CommandParser;
use crate::processor::CommandProcessor;
use crate::server::RESP3Server;
use crate::writer::ResponseWriter;
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

    async fn handle_client(stream: TcpStream) -> std::io::Result<()>{
        let processor = CommandProcessor::new();
        let stream = Arc::new(Mutex::new(stream));

        let parser = CommandParser::new(stream.clone());
        let writer = ResponseWriter::new(stream.clone());

        let mut server = RESP3Server::new(parser, processor, writer);
        server.run().await;
        Ok(())
    }
}
