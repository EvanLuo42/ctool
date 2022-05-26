use data_encoding::Encoding;

pub fn encode_or_decode(action: &str, string: &str, coder: Encoding) {
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
                Err(_) => crate::error::parse_error_exit(),
            }
        },
        _ => crate::error::action_undefined_error_exit(),
    }
}

pub fn encode_or_decode_url(action: &str, string: &str) {
    match action {
        "encode" => println!("Result:\n{}", urlencoding::encode(string)),
        "decode" => println!("Result:\n{}", urlencoding::decode(string).unwrap()),
        _ => crate::error::action_undefined_error_exit(),
    }
}