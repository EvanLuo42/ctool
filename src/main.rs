extern crate core;
extern crate urlencoding;

use clap::{Arg, Command};
use data_encoding::{BASE64, Encoding, HEXLOWER};

fn encode_or_decode(action: &str, string: &str, coder: Encoding) {
    match action {
        "encode" => println!("Result:\n{}", coder.encode(string.as_ref())),
        "decode" => println!("Result:\n{}", coder.decode(string.as_ref()).unwrap_or(Vec::from("[ctool] Unknown string."))
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
    let app = Command::new("ctool");
    let matches = &app
        .version("0.1.0")
        .author("Evan Luo <ziyun.luo@icloud.com>")
        .about("A union tool covert base64, url, etc.")
        .arg(Arg::new("action")
            .short('a')
            .value_name("ACTION")
            .default_value("")
            .help("Action decide whether to encode or decode.")
            .forbid_empty_values(true)
        )
        .arg(Arg::new("type")
            .short('t')
            .value_name("TYPE")
            .default_value("")
            .help("Code type.")
            .forbid_empty_values(true)
        )
        .arg(Arg::new("string")
            .short('s')
            .value_name("STRING")
            .default_value("")
            .help("The string waiting for decode or encode.")
            .forbid_empty_values(true)
        )
        .get_matches();

    let action = matches.value_of("action").unwrap();
    let string = matches.value_of("string").unwrap();
    let kind = matches.value_of("type").unwrap();
    match kind {
        "base64" => encode_or_decode(action, string, BASE64),
        "hex" => encode_or_decode(action, string, HEXLOWER),
        "url" => encode_or_decode_url(action, string),
        _ => println!("Code type undefined."),
    }
}
