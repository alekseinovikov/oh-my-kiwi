use crate::core::command::KiwiCommand;
use crate::core::response::Response;
use crate::core::types::Types;
use crate::core::{CommandProcessor, Engine};
use crate::error::KiwiError;
use async_trait::async_trait;
use std::sync::Arc;

pub(crate) struct KiwiCommandProcessor<E> {
    engine: Arc<E>,
}

#[async_trait]
impl<E> CommandProcessor for KiwiCommandProcessor<E>
where
    E: Engine + Send + Sync,
{
    async fn process(&mut self, command: KiwiCommand) -> Result<Response, KiwiError> {
        self.process(command).await
    }
}

impl<E> KiwiCommandProcessor<E>
where
    E: Engine + Send + Sync,
{
    pub(crate) fn new(engine: Arc<E>) -> Self {
        Self { engine }
    }

    pub(crate) async fn process(&mut self, command: KiwiCommand) -> Result<Response, KiwiError> {
        match command {
            KiwiCommand::None => Ok(Response::Ok),
            KiwiCommand::Ping => Ok(Response::Pong),
            KiwiCommand::Command(_) => Ok(Response::Ok),
            KiwiCommand::Set { key, value } => Ok(self.set(key, value).await),
            KiwiCommand::Get { key } => Ok(self.get(key).await?),
        }
    }

    async fn set(&mut self, key: Types, value: Types) -> Response {
        self.engine.set(key.to_bytes(), value.to_bytes()).await;
        Response::Ok
    }

    async fn get(&self, key: Types) -> Result<Response, KiwiError> {
        match self.engine.get(&key.to_bytes()).await {
            Some(value) => {
                let types = Types::from_slice(value.as_slice()).await?;
                Ok(Response::Value(types))
            }
            None => Ok(Response::Null),
        }
    }
}
