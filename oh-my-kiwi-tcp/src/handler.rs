use oh_my_kiwi_domain::{CommandParser, CommandProcessor, ErrorHandler, ResponseWriter};
use oh_my_kiwi_server::RESP3Server;

pub(super) struct TcpConnectionHandler<P, CP, R, EH> {
    processor: P,
    parser: CP,
    response_writer: R,
    error_handler: EH,
}

impl<P, CP, R, EH> TcpConnectionHandler<P, CP, R, EH>
where
    P: CommandProcessor + Send,
    CP: CommandParser + Send,
    R: ResponseWriter + Send,
    EH: ErrorHandler<R> + Send,
{
    pub(super) fn new(processor: P, parser: CP, response_writer: R, error_handler: EH) -> Self {
        Self {
            processor,
            parser,
            response_writer,
            error_handler,
        }
    }

    pub(super) async fn handle_client(self) -> std::io::Result<()> {
        let processor = self.processor;
        let parser = self.parser;
        let response_writer = self.response_writer;
        let error_handler = self.error_handler;

        let mut server = RESP3Server::new(parser, processor, response_writer, error_handler);
        server.run().await;
        Ok(())
    }
}
