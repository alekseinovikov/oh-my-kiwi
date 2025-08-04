use crate::core::CommandProcessor;
use crate::parser::parser::KiwiCommandParser;
use crate::processor::processor::KiwiCommandProcessor;
use crate::processor::writer::KiwiResponseWriter;
use crate::server::server::RESP3Server;
use crate::transport::tcp::reader::TcpBufferedReader;
use crate::transport::tcp::writer::TcpBytesWriter;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub(super) struct TcpConnectionHandler<Processor> {
    processor: Processor,
}

impl<Processor> TcpConnectionHandler<Processor>
where
    Processor: CommandProcessor + Send,
{
    pub(super) fn new(processor: Processor) -> Self {
        Self { processor }
    }

    pub(super) async fn handle_client(self, stream: TcpStream) -> std::io::Result<()> {
        let processor = self.processor;
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
