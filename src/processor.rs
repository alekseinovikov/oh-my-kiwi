use crate::error::KiwiError;
use crate::parser::Command;
use crate::types::Types;

pub(crate) enum Response {
    Ok,
    Pong,
    Error(String),
}

impl Response {
    pub(crate) fn to_types(&self) -> Types {
        match self {
            Response::Ok => Types::SimpleString("OK".to_string()),
            Response::Pong => Types::SimpleString("PONG".to_string()),
            Response::Error(message) => Types::SimpleError(message.to_string()),
        }
    }
}

pub(crate) struct CommandProcessor;

impl CommandProcessor {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn process(&self, command: Command) -> Result<Response, KiwiError> {
        match command {
            Command::None => Ok(Response::Ok),
            Command::Ping => Ok(Response::Pong),
            Command::Command(arg) => Ok(Response::Ok),
            Command::Set { key, value } => Ok(Response::Ok),
        }
    }
}
