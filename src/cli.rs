use ades::cli::TimeUnit;
use clap::Parser;

/// A CLI tool for compressing and encrypting a directory. (tar, lzma, aes & des)

#[derive(Parser)]
#[clap(version = "0.1.0", author = "hzqd <hzqelf@yeah.net>")]
pub struct Args {
    /// Specify the input file name
    #[clap(short, long)]
    pub input: String,

    /// Specify the output file name
    #[clap(short, long)]
    pub output: String,

    /// Specify the AES key
    #[clap(short, long, default_value = "")]
    pub aes_key: String,

    /// Specify the DES key
    #[clap(short, long, default_value = "")]
    pub des_key: String,

    /// Specify the time unit, support nanos, micros, millis, secs
    #[clap(short, long, default_value = "millis")]
    pub time: TimeUnit,

    /// Set to "compress and encrypt" or "decrypt_and_decompress"
    #[clap(subcommand)]
    pub subcmd: CompressAndEncrypt,
}

#[derive(Parser)]
pub enum CompressAndEncrypt {
    /// A subcommand for specify the mode to "decrypt and decompress" or "compress and encrypt" by -c
    M(Mode),
}

#[derive(Parser)]
pub struct Mode {
    #[clap(short, long)]
    pub compress_and_encrypt: bool
}

pub fn get_args() -> Args {
    Args::parse()
}