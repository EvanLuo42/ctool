use data_encoding::Encoding;
use crate::error;

pub fn encode_or_decode(action: &str, string: &str, coder: Encoding) {
    match action {
        "encode" => println!("Result:\n{}", coder.encode(string.as_ref())),
        "decode" => {
            let result = coder.decode(string.as_ref());
            if let Err(_) = result {
                error::parse_error_exit()
            }

            println!("Result:\n{}", result.unwrap()
                .iter()
                .map(|&c| c as char)
                .collect::<String>())
        },
        _ => error::action_undefined_error_exit(),
    }
}

pub fn encode_or_decode_url(action: &str, string: &str) {
    match action {
        "encode" => println!("Result:\n{}", urlencoding::encode(string)),
        "decode" => println!("Result:\n{}", urlencoding::decode(string).unwrap()),
        _ => error::action_undefined_error_exit(),
    }
}