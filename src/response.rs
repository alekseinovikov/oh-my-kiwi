use crate::types::Types;

pub(crate) enum Response {
    Ok,
    Pong,
    Error(Types),
}
