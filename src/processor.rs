use crate::parser::Command;
use crate::types::Types;

pub(crate) enum Response {
    Ok,
    Pong,
    Error(Types),
}

pub(crate) struct CommandProcessor;

impl CommandProcessor {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn process(&self, command: Command) -> anyhow::Result<Response> {
        match command {
            Command::None => Ok(Response::Ok),
            Command::Ping => Ok(Response::Pong),
            Command::Command(arg) => {}
            Command::Set { key, value } => {}
        }
    }
}
