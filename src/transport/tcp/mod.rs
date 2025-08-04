use crate::core::{CommandParser, CommandProcessor, ResponseWriter};
use crate::transport::tcp::config::TcpConfig;
use crate::transport::tcp::reader::TcpBufferedReader;
use crate::transport::tcp::server::TcpServer;
use crate::transport::tcp::writer::TcpBytesWriter;

pub mod config;
mod handler;
mod reader;
pub(crate) mod server;
mod writer;

pub async fn start_tcp_server<
    Processor,
    ProcessorFactory,
    Parser,
    ParserFactory,
    Responser,
    ResponseWriteFactory,
>(
    processor_factory: ProcessorFactory,
    parser_factory: ParserFactory,
    response_writer_factory: ResponseWriteFactory,
) -> std::io::Result<()>
where
    Processor: CommandProcessor + Send + Sync + 'static,
    ProcessorFactory: Fn() -> Processor + Send + Sync + 'static,
    Parser: CommandParser + Send + Sync + 'static,
    ParserFactory: Fn(TcpBufferedReader) -> Parser + Send + Sync + 'static,
    Responser: ResponseWriter + Send + Sync + 'static,
    ResponseWriteFactory: Fn(TcpBytesWriter) -> Responser + Send + Sync + 'static,
{
    let config = TcpConfig::default();
    let tcp = TcpServer::new(
        config,
        processor_factory,
        parser_factory,
        response_writer_factory,
    );
    tcp.run().await
}
