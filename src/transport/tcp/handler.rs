use crate::core::{CommandParser, CommandProcessor, ResponseWriter};
use crate::server::server::RESP3Server;

pub(super) struct TcpConnectionHandler<Processor, Parser, Responser> {
    processor: Processor,
    parser: Parser,
    response_writer: Responser,
}

impl<Processor, Parser, Responser> TcpConnectionHandler<Processor, Parser, Responser>
where
    Processor: CommandProcessor + Send,
    Parser: CommandParser + Send,
    Responser: ResponseWriter + Send,
{
    pub(super) fn new(processor: Processor, parser: Parser, response_writer: Responser) -> Self {
        Self {
            processor,
            parser,
            response_writer,
        }
    }

    pub(super) async fn handle_client(self) -> std::io::Result<()> {
        let processor = self.processor;
        let parser = self.parser;
        let response_writer = self.response_writer;

        let mut server = RESP3Server::new(parser, processor, response_writer);
        server.run().await;
        Ok(())
    }
}
