use crate::core::{CommandParser, CommandProcessor, ResponseWriter};
use crate::transport::tcp::config::TcpConfig;
use crate::transport::tcp::handler::TcpConnectionHandler;
use crate::transport::tcp::reader::TcpBufferedReader;
use crate::transport::tcp::writer::TcpBytesWriter;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::{error, info};

pub(crate) struct TcpServer<ProcessorFactory, ParserFactory, ResponseWriteFactory> {
    tcp_config: TcpConfig,
    processor_factory: ProcessorFactory,
    parser_factory: ParserFactory,
    response_write_factory: ResponseWriteFactory,
}

impl<ProcessorFactory, ParserFactory, ResponseWriteFactory>
    TcpServer<ProcessorFactory, ParserFactory, ResponseWriteFactory>
{
    pub(crate) fn new(
        tcp_config: TcpConfig,
        processor_factory: ProcessorFactory,
        parser_factory: ParserFactory,
        response_write_factory: ResponseWriteFactory,
    ) -> Self {
        Self {
            tcp_config,
            processor_factory,
            parser_factory,
            response_write_factory,
        }
    }

    pub(crate) async fn run<Processor, Parser, Responser>(&self) -> std::io::Result<()>
    where
        Processor: CommandProcessor + Send + Sync + 'static,
        ProcessorFactory: Fn() -> Processor + Send + Sync + 'static,
        Parser: CommandParser + Send + Sync + 'static,
        ParserFactory: Fn(TcpBufferedReader) -> Parser + Send + Sync + 'static,
        Responser: ResponseWriter + Send + Sync + 'static,
        ResponseWriteFactory: Fn(TcpBytesWriter) -> Responser + Send + Sync + 'static,
    {
        let socket_addr = self.tcp_config.socket_addr()?;
        let listener = TcpListener::bind(socket_addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;
            info!("New connection from: ${addr}");

            let stream = Arc::new(Mutex::new(stream));
            let bytes_reader = TcpBufferedReader::new(stream.clone());
            let bytes_writer = TcpBytesWriter::new(stream.clone());

            let processor = (self.processor_factory)();
            let parser = (self.parser_factory)(bytes_reader);
            let response_writer = (self.response_write_factory)(bytes_writer);
            tokio::spawn(async move {
                let handler = TcpConnectionHandler::new(processor, parser, response_writer);
                if let Err(e) = handler.handle_client().await {
                    error!("Critical error working with connection {}: {:?}", addr, e);
                }
            });
        }
    }
}
