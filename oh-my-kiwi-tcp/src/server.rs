use tokio::net::TcpListener;
use tracing::{error, info, info_span, Instrument};
use oh_my_kiwi_domain::{CommandParser, CommandProcessor, ErrorHandler, ResponseWriter};
use crate::config::TcpConfig;
use crate::handler::TcpConnectionHandler;
use crate::reader::TcpBufferedReader;
use crate::writer::TcpBytesWriter;

pub(crate) struct TcpServer<PF, CPF, RF, EHF> {
    tcp_config: TcpConfig,
    processor_factory: PF,
    parser_factory: CPF,
    response_write_factory: RF,
    error_handler_factory: EHF,
}

impl<PF, CPF, RF, EHF> TcpServer<PF, CPF, RF, EHF> {
    pub(crate) fn new(
        tcp_config: TcpConfig,
        processor_factory: PF,
        parser_factory: CPF,
        response_write_factory: RF,
        error_handler_factory: EHF,
    ) -> Self {
        Self {
            tcp_config,
            processor_factory,
            parser_factory,
            response_write_factory,
            error_handler_factory,
        }
    }

    pub(crate) async fn run<P, CP, R, EH>(&self) -> std::io::Result<()>
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
        let socket_addr = self.tcp_config.socket_addr()?;
        let listener = TcpListener::bind(socket_addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;

            let info_span = info_span!("connection", addr = %addr);
            info!("New connection from: ${addr}");

            let (read_half, write_half) = tokio::io::split(stream);

            let bytes_reader = TcpBufferedReader::new(read_half);
            let bytes_writer = TcpBytesWriter::new(write_half);

            let processor = (self.processor_factory)();
            let parser = (self.parser_factory)(bytes_reader);
            let response_writer = (self.response_write_factory)(bytes_writer);
            let error_handler = (self.error_handler_factory)();
            tokio::spawn(
                async move {
                    let handler = TcpConnectionHandler::new(
                        processor,
                        parser,
                        response_writer,
                        error_handler,
                    );
                    if let Err(e) = handler.handle_client().await {
                        error!("Critical error working with connection {}: {:?}", addr, e);
                    }
                }
                .instrument(info_span),
            );
        }
    }
}
