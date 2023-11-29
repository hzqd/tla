use aoko::no_std::algebraic::sum::TimeUnit;
use clap::Parser;

/// A CLI tool for compressing and encrypting a file or directory. (tar, lzma, aes & des)

#[derive(Parser)]
#[clap(version = "0.2.0", author = "hzqd <hzqelf@yeah.net>")]
pub struct Args {
    /// Specify the input file name
    #[clap()]
    pub input: String,

    /// Specify the Caesar key
    #[clap(short, long, default_value_t = 0)]
    pub caesar: u8,

    /// Specify the AES key
    #[clap(short, long, default_value = "")]
    pub aes_key: String,

    /// Specify the DES key
    #[clap(short, long, default_value = "")]
    pub des_key: String,

    /// Specify the RSA private key file path
    #[clap(short, long, default_value = "")]
    pub rsa_private_key: String,

    /// Use it to select compress, decompress, or no compress
    #[clap(subcommand)]
    pub compress: Compress,

    /// Specify the time unit, support nanos, micros, millis, secs
    #[clap(short, long, default_value = "millis")]
    pub time: TimeUnit,
}

#[derive(Parser)]
pub enum Compress {
    /// A subcommand for specify using LZMA Compress
    C,
    /// A subcommand for specify to Decompress
    D,
    /// A subcommand for specify No Compress
    N(Tar),
}

#[derive(Parser)]
pub struct Tar {
    /// Use it to tar (and encrypt), omit it to unpack (and decrypt)
    #[clap(short, long)]
    pub tar: bool
}

pub fn get_args() -> Args {
    Args::parse()
}