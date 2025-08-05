use crate::core::types::Types;
use crate::core::{command, BytesReader, CommandParser};
use crate::error::{CommandError, KiwiError};
use async_trait::async_trait;
pub(crate) use command::KiwiCommand;

pub(crate) struct KiwiCommandParser<Reader: BytesReader + Send> {
    reader: Reader,
}

#[async_trait]
impl<Reader: BytesReader + Send> CommandParser for KiwiCommandParser<Reader> {
    async fn parse_next_command(&mut self) -> Result<KiwiCommand, KiwiError> {
        self.parse_next_command().await
    }
}

impl<Reader: BytesReader + Send> KiwiCommandParser<Reader> {
    pub(crate) fn new(bytes_reader: Reader) -> Self {
        Self {
            reader: bytes_reader,
        }
    }

    pub(crate) async fn parse_next_command(&mut self) -> Result<KiwiCommand, KiwiError> {
        let types = Types::from_bytes(&mut self.reader).await?;
        Ok(Self::parse_command_from_types(types)?)
    }

    fn parse_command_from_types(types: Types) -> Result<KiwiCommand, CommandError> {
        match types {
            Types::Array(values) => Self::parse_command(values),
            _ => Err(CommandError::UnsupportedCommand),
        }
    }

    fn parse_command(mut args: Vec<Types>) -> Result<KiwiCommand, CommandError> {
        if args.is_empty() {
            return Err(CommandError::UnsupportedCommand);
        }

        let command_name = args.remove(0);
        match command_name {
            Types::BulkString(str) => KiwiCommand::parse_command(str.as_str(), args),
            _ => Err(CommandError::UnsupportedCommand),
        }
    }
}
