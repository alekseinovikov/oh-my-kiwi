use async_trait::async_trait;
use crate::core::command::KiwiCommand;
use crate::core::error::{KiwiError, ParseError};
use crate::core::response::Response;

pub(crate) mod command;
pub mod error;
pub mod response;
pub mod types;

pub(crate) trait CommandParser {
    async fn parse_next_command(&mut self) -> Result<KiwiCommand, KiwiError>;
}

#[async_trait]
pub(crate) trait CommandProcessor {
    async fn process(&self, command: KiwiCommand) -> Result<Response, KiwiError>;
}

pub(crate) trait ResponseWriter {
    async fn write(&mut self, response: Response) -> Result<(), KiwiError>;
}

pub(crate) trait BytesReader {
    async fn read_line(&mut self) -> Result<Vec<u8>, ParseError>;
    async fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, ParseError>;
}

pub(crate) trait BytesWriter {
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), KiwiError>;
}
