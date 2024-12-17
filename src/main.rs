use base64::{prelude::BASE64_STANDARD, Engine};
use clap::Parser;
use const_hex::ToHexExt;
use derive_more::derive::{Display, FromStr};
use rand::{thread_rng, Rng};

fn main() {
    Cli::parse().run()
}

/// Secret generator.
#[derive(Debug, Parser)]
#[command()]
struct Cli {
    /// Encoding to be used.
    #[arg(short, long, default_value_t)]
    encoding: Encoding,

    /// Number of bytes to be used.
    len: usize,
}

impl Cli {
    fn run(self) {
        let Cli { len, encoding } = self;

        let mut bytes = vec![0u8; len];
        thread_rng().fill(bytes.as_mut_slice());

        let encoded = match encoding {
            Encoding::Base64 => BASE64_STANDARD.encode(bytes),
            Encoding::Hex => bytes.encode_hex(),
        };

        print!("{encoded}");
    }
}

#[derive(Debug, Clone, Default, Display, FromStr)]
enum Encoding {
    #[default]
    Hex,
    Base64,
}
