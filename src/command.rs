use crate::reader::{BufferedReader, Token};
use crate::types::Types;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Debug)]
pub(crate) enum Command {
    None,
    Ping,
    Command(String),
    Set { key: Types, value: Types },
}

pub(crate) struct CommandParser {
    stream: Arc<Mutex<TcpStream>>,
    reader: BufferedReader,
}

impl CommandParser {
    pub(crate) fn new(stream: TcpStream) -> Self {
        let stream = Arc::new(Mutex::new(stream));
        let reader = BufferedReader::new(stream.clone());
        Self { stream, reader }
    }

    pub(crate) async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let tokens = self.read_command_tokens().await?;
            if let Ok(command) = Self::parse_command_from_tokens(tokens) {
                info!("{:?}", command);
            }
        }
    }

    async fn read_command_tokens(&mut self) -> anyhow::Result<Vec<Token>> {
        let mut array_def = self.reader.get_next_token().await;
        if array_def[0] != b'*' {
            return Err(anyhow::anyhow!("Invalid command"));
        }

        let array_size = array_def.split_off(1);
        let array_size = std::str::from_utf8(&array_size)?;
        let array_size: usize = array_size.parse()?;
        let mut tokens: Vec<Token> = Vec::with_capacity(array_size);
        for _ in 0..array_size {
            let new_token = self.read_next_command_token().await?;
            tokens.push(new_token);
        }

        Ok(tokens)
    }

    async fn read_next_command_token(&mut self) -> anyhow::Result<Token> {
        let token_def = self.reader.get_next_token().await;
        let token_type = token_def[0];
        match token_type {
            b'$' => self.read_next_string(token_def).await,
            _ => Err(anyhow::anyhow!("Invalid command")),
        }
    }

    async fn read_next_string(&mut self, mut string_def: Vec<u8>) -> anyhow::Result<Token> {
        let string_size = string_def.split_off(1);
        let string_size = std::str::from_utf8(&string_size)?;
        let string_size: usize = string_size.parse()?;

        let next_token = self.reader.get_next_token().await;
        if next_token.len() != string_size {
            return Err(anyhow::anyhow!("String doesn't match the size"));
        }

        let string = std::str::from_utf8(&next_token)?;
        Ok(Token::String(string.to_string()))
    }

    fn parse_command_from_tokens(mut tokens: Vec<Token>) -> anyhow::Result<Command> {
        if tokens.is_empty() {
            return Err(anyhow::anyhow!("Invalid command"));
        }

        let args = tokens.split_off(1);
        let command_token = &tokens[0];
        match command_token {
            Token::String(name) => Self::parse_command(name.to_uppercase(), args),
            _ => Err(anyhow::anyhow!("Invalid command")),
        }
    }

    fn parse_command(name: String, args: Vec<Token>) -> anyhow::Result<Command> {
        match name.as_str() {
            "PING" => Ok(Command::Ping),
            "COMMAND" => Self::build_command_command(args),
            _ => Err(anyhow::anyhow!("Invalid command")),
        }
    }

    fn build_command_command(args: Vec<Token>) -> anyhow::Result<Command> {
        if args.is_empty() || args.len() > 1 {
            Err(anyhow::anyhow!("Invalid command"))
        } else {
            match &args[0] {
                Token::String(arg) => Ok(Command::Command(arg.to_string())),
                _ => Err(anyhow::anyhow!("Invalid command")),
            }
        }
    }
}
