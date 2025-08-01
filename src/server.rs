use crate::error::KiwiError;
use crate::parser::CommandParser;
use crate::processor::{CommandProcessor, Response};
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
    pub(crate) async fn run(&mut self) {
        loop {
            let run_result = self.run_once().await;

            if let Err(err) = run_result {
                let handle_error_result = self.handle_kiwi_error(err).await;
                if let Err(error) = handle_error_result {
                    error!("Fatal error: {:?} Closing connection...", error);
                    break;
                }
            }
        }
    }

    async fn run_once(&mut self) -> Result<(), KiwiError> {
        let command = self.parser.parse_next_command().await?;
        let response = self.processor.process(command).await?;
        self.writer.write(response).await
    }

    async fn handle_kiwi_error(&mut self, err: KiwiError) -> Result<(), KiwiError> {
        let response = Self::map_err_to_response_if_possible(err)?;
        self.write_error_response(response).await
    }

    async fn write_error_response(&mut self, response: Response) -> Result<(), KiwiError> {
        self.writer.write(response).await
    }

    fn map_err_to_response_if_possible(err: KiwiError) -> Result<Response, KiwiError> {
        match err {
            KiwiError::ParseError(err) => Ok(Response::Error(err.to_string())),
            KiwiError::CommandError(err) => Ok(Response::Error(err.to_string())),
            KiwiError::ConnectionError(_) => Err(err),
            KiwiError::UnexpectedError => Err(err),
        }
    }
}
