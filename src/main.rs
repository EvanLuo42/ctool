mod error;
mod encoding;

extern crate core;
extern crate urlencoding;

use clap::{Arg, Command};
use data_encoding::{BASE64, HEXLOWER};

fn command() -> Command<'static> {
    Command::new("ctool")
}

fn main() {
    let matches = command()
        .version("0.1.0")
        .author("Evan Luo <ziyun.luo@icloud.com>")
        .about("A union tool covert base64, url, etc.")
        .arg(Arg::new("action")
            .short('a')
            .value_name("ACTION")
            .help("Action decide whether to encode or decode.")
            .required(true)
        )
        .arg(Arg::new("type")
            .short('t')
            .value_name("TYPE")
            .help("Code type.")
            .required(true)
        )
        .arg(Arg::new("string")
            .short('s')
            .value_name("STRING")
            .help("The string waiting for decode or encode.")
            .required(true)
        )
        .try_get_matches_from(std::env::args_os())
        .unwrap_or_else(|e| e.exit());

    let action = matches.value_of("action").unwrap();
    let string = matches.value_of("string").unwrap();
    let kind = matches.value_of("type").unwrap();

    match kind {
        "base64" => encoding::encode_or_decode(action, string, BASE64),
        "hex" => encoding::encode_or_decode(action, string, HEXLOWER),
        "url" => encoding::encode_or_decode_url(action, string),
        _ => error::type_undefined_error_exit(),
    }
}
