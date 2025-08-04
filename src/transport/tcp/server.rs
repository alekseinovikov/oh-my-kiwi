use crate::core::CommandProcessor;
use crate::parser::parser::KiwiCommandParser;
use crate::processor::processor::KiwiCommandProcessor;
use crate::processor::writer::KiwiResponseWriter;
use crate::server::server::RESP3Server;
use crate::transport::tcp::config::TcpConfig;
use crate::transport::tcp::handler::TcpConnectionHandler;
use crate::transport::tcp::reader::TcpBufferedReader;
use crate::transport::tcp::writer::TcpBytesWriter;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::{error, info};

pub(crate) struct TcpServer<ProcessorFactory> {
    tcp_config: TcpConfig,
    processor_factory: ProcessorFactory,
}

impl<ProcessorFactory, Processor> TcpServer<ProcessorFactory>
where
    Processor: CommandProcessor + Send + Sync + 'static,
    ProcessorFactory: Fn() -> Processor + Send + Sync + 'static,
{
    pub(crate) fn new(tcp_config: TcpConfig, processor_factory: ProcessorFactory) -> Self {
        Self {
            tcp_config,
            processor_factory,
        }
    }

    pub(crate) async fn run(&self) -> std::io::Result<()> {
        let socket_addr = self.tcp_config.socket_addr()?;
        let listener = TcpListener::bind(socket_addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;
            info!("New connection from: ${addr}");

            let processor = (self.processor_factory)();
            tokio::spawn(async move {
                let handler = TcpConnectionHandler::new(processor);

                if let Err(e) = handler.handle_client(stream).await {
                    error!("Error with {}: {:?}", addr, e);
                }
            });
        }
    }
}
