#![doc = include_str!("../README.md")]

#[deny(legacy_derive_helpers)]
mod error;

use std::io::Read;

use clap::Parser;

#[derive(Debug, clap::ValueEnum, PartialEq, Eq, Clone, Copy)]
enum Action {
    Encode,
    Decode,
}

#[derive(Debug, clap::ValueEnum, PartialEq, Eq, Clone, Copy)]
enum Ty {
    Base64,
    Hex,
    Url,
}

#[derive(Debug, Parser)]
#[clap(
    version = clap::crate_version!(),
    author = clap::crate_authors!("\n"),
    about = "A union tool covert base64, url, etc.",
)]
struct Opt {
    #[clap(
        long,
        short = 'a',
        value_enum,
        help = "Action decide whether to encode or decode.",
        required = true
    )]
    action: Action,
    #[clap(long, short = 't', value_enum, help = "Code type.", required = true)]
    ty: Ty,
    #[clap(last = true, help = "The string waiting for decode or encode.")]
    string: Option<String>,
}

impl Opt {
    /// do the action
    pub fn do_action(self) -> anyhow::Result<String> {
        match self.action {
            Action::Encode => self.encode(),
            Action::Decode => self.decode(),
        }
    }

    fn encode(self) -> anyhow::Result<String> {
        match self.ty {
            Ty::Base64 => Err(anyhow::Error::from(error::Error::Todo)),
            Ty::Hex => Err(anyhow::Error::from(error::Error::Todo)),
            Ty::Url => Err(anyhow::Error::from(error::Error::Todo)),
        }
    }

    fn decode(self) -> anyhow::Result<String> {
        match self.ty {
            Ty::Base64 => Err(anyhow::Error::from(error::Error::Todo)),
            Ty::Hex => Err(anyhow::Error::from(error::Error::Todo)),
            Ty::Url => Err(anyhow::Error::from(error::Error::Todo)),
        }
    }
}

fn strip(s: String) -> String {
    // remove bom
    let s = s.replace('\u{feff}', "");
    // trim end
    s.trim_end().to_string()
}

fn main() -> anyhow::Result<()> {
    let mut args: Opt = Opt::parse();
    let mut buffer = String::new();
    if args.string == None {
        std::io::stdin().read_to_string(&mut buffer)?;
        args.string = Some(strip(buffer))
    }

    // println!("{:#?}", args);
    // code to do with args
    println!("{}", args.do_action()?);
    Ok(())
}
