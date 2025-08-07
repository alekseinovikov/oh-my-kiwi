use crate::core::types::Types;

pub(crate) enum Response {
    Ok,
    Pong,
    Value(Types),
    Error(String),
    Null
}

impl Response {
    pub(crate) fn to_types(&self) -> Types {
        match self {
            Response::Ok => Types::SimpleString("OK".to_string()),
            Response::Pong => Types::SimpleString("PONG".to_string()),
            Response::Error(message) => Types::SimpleError(message.to_string()),
            Response::Value(types) => types.clone(),
            Response::Null => Types::Null
        }
    }
}
