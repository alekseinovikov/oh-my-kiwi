use crate::error::{CommandError, KiwiError};
use crate::reader::BufferedReader;
use crate::types::Types;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[derive(Debug)]
pub(crate) enum Command {
    None,
    Ping,
    Command(String),
    Set { key: Types, value: Types },
}

impl Command {
    pub(crate) fn parse_command(name: &str, args: Vec<Types>) -> Result<Command, CommandError> {
        let binding = name.to_uppercase();
        let name = binding.as_str();
        match name {
            "PING" => Ok(Command::Ping),
            "COMMAND" => Self::create_command(args),
            _ => Err(CommandError::UnsupportedCommand),
        }
    }

    fn create_command(args: Vec<Types>) -> Result<Command, CommandError> {
        if args.is_empty() || args.len() > 1 {
            Err(CommandError::WrongNumberOfArguments)
        } else if let Types::BulkString(key) = &args[0] {
            Ok(Command::Command(key.to_string()))
        } else {
            Err(CommandError::WrongArgumentType)
        }
    }
}

pub(crate) struct CommandParser {
    reader: BufferedReader,
}

impl CommandParser {
    pub(crate) fn new(stream: Arc<Mutex<TcpStream>>) -> Self {
        let reader = BufferedReader::new(stream);
        Self { reader }
    }

    pub(crate) async fn parse_next_command(&mut self) -> Result<Command, KiwiError> {
        let types = Types::from_stream(&mut self.reader).await?;
        Ok(Self::parse_command_from_types(types)?)
    }

    fn parse_command_from_types(types: Types) -> Result<Command, CommandError> {
        match types {
            Types::Array(values) => Self::parse_command(values),
            _ => Err(CommandError::UnsupportedCommand),
        }
    }

    fn parse_command(mut args: Vec<Types>) -> Result<Command, CommandError> {
        if args.is_empty() {
            return Err(CommandError::UnsupportedCommand);
        }

        let command_name = args.remove(0);
        match command_name {
            Types::BulkString(str) => Command::parse_command(str.as_str(), args),
            _ => Err(CommandError::UnsupportedCommand),
        }
    }
}
