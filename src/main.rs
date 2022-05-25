extern crate core;
extern crate urlencoding;

use clap::{Arg, Command};
use data_encoding::{BASE64, Encoding, HEXLOWER};

fn encode_or_decode(action: &str, string: &str, coder: Encoding) {
    let action_undefined_error = command().error(
        clap::ErrorKind::InvalidValue,
        "The given action is not supported, please enter encode or decode."
    );
    match action {
        "encode" => println!("Result:\n{}", coder.encode(string.as_ref())),
        "decode" => {
            match coder.decode(string.as_ref()) {
                Ok(code) => {
                    println!("Result:\n{}", code
                        .iter()
                        .map(|&c| c as char)
                        .collect::<String>())
                },
                Err(_) => {
                    let parse_error = command().error(
                        clap::ErrorKind::InvalidValue,
                        "The given string is not a base64 or hex."
                    );
                    parse_error.exit()
                }
            }
        },
        _ => action_undefined_error.exit(),
    }
}

fn encode_or_decode_url(action: &str, string: &str) {
    let action_undefined_error = command().error(
        clap::ErrorKind::InvalidValue,
        "The given action is not supported, please enter encode or decode."
    );
    match action {
        "encode" => println!("Result:\n{}", urlencoding::encode(string)),
        "decode" => println!("Result:\n{}", urlencoding::decode(string).unwrap()),
        _ => action_undefined_error.exit(),
    }
}

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

    let type_undefined_error = command().error(
        clap::ErrorKind::InvalidValue,
        "The given type is not supported, please enter base64, hex, etc."
    );

    let action = matches.value_of("action").unwrap();
    let string = matches.value_of("string").unwrap();
    let kind = matches.value_of("type").unwrap();

    match kind {
        "base64" => encode_or_decode(action, string, BASE64),
        "hex" => encode_or_decode(action, string, HEXLOWER),
        "url" => encode_or_decode_url(action, string),
        _ => type_undefined_error.exit(),
    }
}
