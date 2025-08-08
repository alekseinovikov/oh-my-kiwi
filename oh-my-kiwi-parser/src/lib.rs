use async_trait::async_trait;
use oh_my_kiwi_domain::command::KiwiCommand;
use oh_my_kiwi_domain::error::{CommandError, KiwiError};
use oh_my_kiwi_domain::types::Types;
use oh_my_kiwi_domain::{BytesReader, CommandParser};

pub struct KiwiCommandParser<Reader: BytesReader + Send> {
    reader: Reader,
}

#[async_trait]
impl<Reader: BytesReader + Send> CommandParser for KiwiCommandParser<Reader> {
    async fn parse_next_command(&mut self) -> Result<KiwiCommand, KiwiError> {
        self.parse_next_command().await
    }
}

impl<Reader: BytesReader + Send> KiwiCommandParser<Reader> {
    pub fn new(bytes_reader: Reader) -> Self {
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
