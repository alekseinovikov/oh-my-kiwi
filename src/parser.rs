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
    pub(crate) fn parse_command(name: &str, args: Vec<Types>) -> anyhow::Result<Command> {
        Err(anyhow::anyhow!("Not implemented"))
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

    pub(crate) async fn parse_next_command(&mut self) -> anyhow::Result<Command> {
        let types = Types::from_stream(&mut self.reader).await?;
        Self::parse_command_from_tokens(types)
    }

    fn parse_command_from_tokens(types: Types) -> anyhow::Result<Command> {
        match types {
            Types::Array(values) => Self::parse_command(values),
            _ => Err(anyhow::anyhow!("Command must be an array")),
        }
    }

    fn parse_command(mut args: Vec<Types>) -> anyhow::Result<Command> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("Command array is empty"));
        }

        let command_name = args.remove(0);
        match command_name {
            Types::BulkString(str) => Command::parse_command(str.as_str(), args),
            _ => Err(anyhow::anyhow!("Command must start from name")),
        }
    }
}
