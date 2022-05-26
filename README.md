# Ctool
An encoding and decoding CLI tool written in Rust.

Now provided:
- Base64
- Hex (Lower case)
- Url

---
### Installation:
1. Download the file in release.
2. Put it into /usr/local/bin/

Or using `cargo build --release` to generate file.

---
### Usage:
```
ctool -a [encode | decode] -t [base64 | hex | url] -s <STRING>
```

---
### Acknowledgements：
- [Clap](https://github.com/clap-rs/clap)
- [Data-Encoding](https://github.com/ia0/data-encoding)
- [UrlEncoding](https://github.com/bt/rust_urlencoding)
