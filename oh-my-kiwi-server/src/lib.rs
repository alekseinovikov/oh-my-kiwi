use tracing::error;
use oh_my_kiwi_domain::{CommandParser, CommandProcessor, ErrorHandler, ResponseWriter};
use oh_my_kiwi_domain::error::KiwiError;

pub struct RESP3Server<CP, P, W, EH>
where
    CP: CommandParser,
    P: CommandProcessor,
    W: ResponseWriter,
    EH: ErrorHandler<W>,
{
    parser: CP,
    processor: P,
    writer: W,
    error_handler: EH,
}

impl<CP, P, W, EH> RESP3Server<CP, P, W, EH>
where
    CP: CommandParser,
    P: CommandProcessor,
    W: ResponseWriter,
    EH: ErrorHandler<W>,
{
    pub const fn new(parser: CP, processor: P, writer: W, error_handler: EH) -> Self {
        Self {
            parser,
            processor,
            writer,
            error_handler,
        }
    }
}

impl<CP, P, W, EH> RESP3Server<CP, P, W, EH>
where
    CP: CommandParser,
    P: CommandProcessor,
    W: ResponseWriter,
    EH: ErrorHandler<W>,
{
    pub async fn run(&mut self) {
        loop {
            let run_result = self.run_once().await;

            if let Err(err) = run_result {
                let handle_error_result =
                    self.error_handler.handle_error(&mut self.writer, err).await;
                if let Some(error) = handle_error_result {
                    if let KiwiError::ConnectionClosed = error {
                        break;
                    }

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
}
