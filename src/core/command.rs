use crate::core::types::Types;
use crate::error::CommandError;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum KiwiCommand {
    None,
    Ping,
    Command(String),
    Set { key: Types, value: Types },
    Get { key: Types },
}

impl KiwiCommand {
    pub(crate) fn parse_command(name: &str, args: Vec<Types>) -> Result<KiwiCommand, CommandError> {
        let binding = name.to_uppercase();
        let name = binding.as_str();
        match name {
            "PING" => Ok(KiwiCommand::Ping),
            "COMMAND" => Self::create_command(args),
            "GET" => Self::create_get(args),
            "SET" => Self::create_set(args),
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

    fn create_get(args: Vec<Types>) -> Result<KiwiCommand, CommandError> {
        if args.len() != 1 {
            Err(CommandError::WrongNumberOfArguments)
        } else {
            Ok(KiwiCommand::Get {
                key: args[0].clone(),
            })
        }
    }

    fn create_set(args: Vec<Types>) -> Result<KiwiCommand, CommandError> {
        if args.len() != 2 {
            Err(CommandError::WrongNumberOfArguments)
        } else {
            Ok(KiwiCommand::Set {
                key: args[0].clone(),
                value: args[1].clone(),
            })
        }
    }
}
