use crate::core::error::CommandError;
use crate::core::types::Types;

#[derive(Debug)]
pub(crate) enum KiwiCommand {
    None,
    Ping,
    Command(String),
    Set { key: Types, value: Types },
}

impl KiwiCommand {
    pub(crate) fn parse_command(name: &str, args: Vec<Types>) -> Result<KiwiCommand, CommandError> {
        let binding = name.to_uppercase();
        let name = binding.as_str();
        match name {
            "PING" => Ok(KiwiCommand::Ping),
            "COMMAND" => Self::create_command(args),
            _ => Err(CommandError::UnsupportedCommand),
        }
    }

    fn create_command(args: Vec<Types>) -> Result<KiwiCommand, CommandError> {
        if args.is_empty() || args.len() > 1 {
            Err(CommandError::WrongNumberOfArguments)
        } else if let Types::BulkString(key) = &args[0] {
            Ok(KiwiCommand::Command(key.to_string()))
        } else {
            Err(CommandError::WrongArgumentType)
        }
    }
}
