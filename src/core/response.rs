use crate::core::types::Types;

pub(crate) enum Response {
    Ok,
    Pong,
    Error(String),
}

impl Response {
    pub(crate) fn to_types(&self) -> Types {
        match self {
            Response::Ok => Types::SimpleString("OK".to_string()),
            Response::Pong => Types::SimpleString("PONG".to_string()),
            Response::Error(message) => Types::SimpleError(message.to_string()),
        }
    }
}
