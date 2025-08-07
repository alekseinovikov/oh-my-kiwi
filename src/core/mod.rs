use crate::core::command::KiwiCommand;
use crate::core::response::Response;
use crate::error::{KiwiError, ParseError};
use async_trait::async_trait;

pub(crate) mod command;
pub mod response;
pub mod types;

#[async_trait]
pub(crate) trait CommandParser {
    async fn parse_next_command(&mut self) -> Result<KiwiCommand, KiwiError>;
}

#[async_trait]
pub(crate) trait CommandProcessor {
    async fn process(&mut self, command: KiwiCommand) -> Result<Response, KiwiError>;
}

#[async_trait]
pub(crate) trait ResponseWriter {
    async fn write(&mut self, response: Response) -> Result<(), KiwiError>;
}

#[async_trait]
pub(crate) trait BytesReader {
    async fn read_line(&mut self) -> Result<Vec<u8>, ParseError>;
    async fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, ParseError>;
}

#[async_trait]
pub(crate) trait BytesWriter {
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), KiwiError>;
}

#[async_trait]
pub(crate) trait ErrorHandler<RW> {
    async fn handle_error(&self, response_writer: &mut RW, error: KiwiError) -> Option<KiwiError>;
}

#[async_trait]
pub(crate) trait Engine {
    async fn get(&self, key: &Vec<u8>) -> Option<Vec<u8>>;
    async fn set(&self, key: Vec<u8>, value: Vec<u8>);
}
