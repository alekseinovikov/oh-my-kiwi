use crate::core::CommandProcessor;
use crate::core::command::KiwiCommand;
use crate::core::error::KiwiError;
use crate::core::response::Response;

pub(crate) struct KiwiCommandProcessor;

impl CommandProcessor for KiwiCommandProcessor {
    async fn process(&self, command: KiwiCommand) -> Result<Response, KiwiError> {
        self.process(command).await
    }
}

impl KiwiCommandProcessor {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn process(&self, command: KiwiCommand) -> Result<Response, KiwiError> {
        match command {
            KiwiCommand::None => Ok(Response::Ok),
            KiwiCommand::Ping => Ok(Response::Pong),
            KiwiCommand::Command(arg) => Ok(Response::Ok),
            KiwiCommand::Set { key, value } => Ok(Response::Ok),
        }
    }
}
