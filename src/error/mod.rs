pub(crate) mod handler;

use std::num::{ParseFloatError, ParseIntError};
use std::string::FromUtf8Error;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum KiwiError {
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),

    #[error("Command error: {0}")]
    CommandError(#[from] CommandError),

    #[error("Connection error: {0}")]
    ConnectionError(#[from] std::io::Error),

    #[error("Connection closed")]
    ConnectionClosed,
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Unsupported command")]
    UnsupportedCommand,

    #[error("Wrong number of arguments")]
    WrongNumberOfArguments,

    #[error("Wrong argument type")]
    WrongArgumentType,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Expected number, got {0}")]
    ExpectedNumber(#[from] ParseIntError),

    #[error("Wrong string byte sequence")]
    WrongStringByteSequence(#[from] FromUtf8Error),

    #[error("Wrong big number format")]
    WrongBigNumberFormat,

    #[error("Wrong floating point format")]
    WrongFloatingPointFormat(#[from] ParseFloatError),

    #[error("Expected boolean")]
    ExpectedBool,

    #[error("Missing CLRF separator")]
    MissingSeparator,

    #[error("Unsupported data type {0}")]
    UnsupportedDataType(String),

    #[error("Client closed connection")]
    ConnectionClosed,

    #[error("Connection error: {0}")]
    ConnectionError(#[from] std::io::Error),
}
