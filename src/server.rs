use crate::parser::CommandParser;
use crate::processor::CommandProcessor;
use crate::writer::ResponseWriter;
use tracing::error;

pub(crate) struct RESP3Server {
    parser: CommandParser,
    processor: CommandProcessor,
    writer: ResponseWriter,
}

impl RESP3Server {
    pub(crate) const fn new(
        parser: CommandParser,
        processor: CommandProcessor,
        writer: ResponseWriter,
    ) -> Self {
        Self {
            parser,
            processor,
            writer,
        }
    }
}

impl RESP3Server {
    pub(crate) async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let command = self.parser.parse_next_command().await;
            match command {
                Ok(command) => {
                    let response = self.processor.process(command).await;
                    match response {
                        Ok(response) => self.writer.write(response).await?,
                        Err(err) => {
                            error!(?err, "Failed to process command")
                        }
                    }
                }
                Err(err) => {
                    error!(?err, "Failed to parse command")
                }
            }
        }
    }
}
