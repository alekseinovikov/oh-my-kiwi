use crate::core::{CommandParser, CommandProcessor, ErrorHandler, ResponseWriter};
use crate::transport::tcp::config::TcpConfig;
use crate::transport::tcp::reader::TcpBufferedReader;
use crate::transport::tcp::server::TcpServer;
use crate::transport::tcp::writer::TcpBytesWriter;

pub mod config;
mod handler;
mod reader;
pub(crate) mod server;
mod writer;

pub async fn start_tcp_server<P, PF, CP, CPF, R, RF, EH, EHF>(
    processor_factory: PF,
    parser_factory: CPF,
    response_writer_factory: RF,
    error_handler_factory: EHF,
) -> std::io::Result<()>
where
    P: CommandProcessor + Send + Sync + 'static,
    PF: Fn() -> P + Send + Sync + 'static,
    CP: CommandParser + Send + Sync + 'static,
    CPF: Fn(TcpBufferedReader) -> CP + Send + Sync + 'static,
    R: ResponseWriter + Send + Sync + 'static,
    RF: Fn(TcpBytesWriter) -> R + Send + Sync + 'static,
    EH: ErrorHandler<R> + Send + Sync + 'static,
    EHF: Fn() -> EH + Send + Sync + 'static,
{
    let config = TcpConfig::default();
    let tcp = TcpServer::new(
        config,
        processor_factory,
        parser_factory,
        response_writer_factory,
        error_handler_factory,
    );
    tcp.run().await
}
