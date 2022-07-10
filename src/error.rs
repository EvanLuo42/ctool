use std::fmt::Display;

use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("The given string \"{0}\" is not a base64.")]
    ParseErrorBase64(String),
    #[error("The given string \"{0}\" is not a hex.")]
    ParseErrorBaseHex(String),
    #[error("todo")]
    Todo,
}
