extern crate core;
extern crate urlencoding;

use clap::{Arg, Command};
use data_encoding::{BASE64, Encoding, HEXLOWER};

fn encode_or_decode(action: &str, string: &str, coder: Encoding) {
    match action {
        "encode" => println!("Result:\n{}", coder.encode(string.as_ref())),
        "decode" => println!("Result:\n{}", coder.decode(string.as_ref()).expect("[ctool] Unknown string.\n")
            .iter()
            .map(|&c| c as char)
            .collect::<String>()),
        _ => println!("[ctool] Action undefined."),
    }
}

fn encode_or_decode_url(action: &str, string: &str) {
    match action {
        "encode" => println!("Result:\n{}", urlencoding::encode(string)),
        "decode" => println!("Result:\n{}", urlencoding::decode(string).unwrap()),
        _ => println!("[ctool] Action undefined."),
    }
}

fn main() {
    let matches = Command::new("ctool")
        .version("0.1.0")
        .author("Evan Luo <ziyun.luo@icloud.com>")
        .about("A union tool covert base64, url, etc.")
        .arg(Arg::new("action")
            .short('a')
            .value_name("ACTION")
            .help("Action decide whether to encode or decode.")
            .forbid_empty_values(false)
        )
        .arg(Arg::new("type")
            .short('t')
            .value_name("TYPE")
            .help("Code type.")
            .forbid_empty_values(false)
        )
        .arg(Arg::new("string")
            .short('s')
            .value_name("STRING")
            .help("The string waiting for decode or encode.")
        )
        .get_matches();

    match matches.value_of("action") {
        Some(action) => {
            match matches.value_of("string") {
                Some(string) => {
                    match matches.value_of("type") {
                        Some(kind) => {
                            match kind {
                                "base64" => encode_or_decode(action, string, BASE64),
                                "hex" => encode_or_decode(action, string, HEXLOWER),
                                "url" => encode_or_decode_url(action, string),
                                _ => println!("[ctool] Code type undefined."),
                            }
                        },
                        None => println!("[ctool] The given argument type have to be provided.")
                    }
                },
                None => println!("[ctool] The given argument string have to be provided.")
            }
        },
        None => println!("[ctool] The given argument action have to be provided.")
    }
}
