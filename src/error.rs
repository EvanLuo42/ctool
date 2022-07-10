use std::fmt::Display;
#[derive(Debug)]
pub enum Error {
    ParseErrorBase64,
    ParseErrorBaseHex,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseErrorBase64 => f.write_str("The given string is not a base64."),
            Error::ParseErrorBaseHex => f.write_str("The given string is not a hex."),
        }
    }
}
