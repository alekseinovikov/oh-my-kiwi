use async_trait::async_trait;
use crate::command::KiwiCommand;
use crate::error::{KiwiError, ParseError};
use crate::response::Response;

pub mod command;
pub mod response;
pub mod types;
pub mod error;

#[async_trait]
pub trait CommandParser {
    async fn parse_next_command(&mut self) -> Result<KiwiCommand, KiwiError>;
}

#[async_trait]
pub trait CommandProcessor {
    async fn process(&mut self, command: KiwiCommand) -> Result<Response, KiwiError>;
}

#[async_trait]
pub trait ResponseWriter {
    async fn write(&mut self, response: Response) -> Result<(), KiwiError>;
}

#[async_trait]
pub trait BytesReader {
    async fn read_line(&mut self) -> Result<Vec<u8>, ParseError>;
    async fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, ParseError>;
}

#[async_trait]
pub trait BytesWriter {
    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), KiwiError>;
}

#[async_trait]
pub trait ErrorHandler<RW> {
    async fn handle_error(&self, response_writer: &mut RW, error: KiwiError) -> Option<KiwiError>;
}

#[async_trait]
pub trait Engine {
    async fn get(&self, key: &Vec<u8>) -> Option<Vec<u8>>;
    async fn set(&self, key: Vec<u8>, value: Vec<u8>);
}
