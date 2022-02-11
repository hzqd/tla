use ades::cli::TimeUnit;
use clap::Parser;

/// A CLI tool for compressing and encrypting a file or directory. (tar, lzma, aes & des)

#[derive(Parser)]
#[clap(version = "0.1.2", author = "hzqd <hzqelf@yeah.net>")]
pub struct Args {
    /// Specify the input file name
    #[clap(short, long)]
    pub input: String,

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
    pub subcmd: Mode,
}

#[derive(Parser)]
pub enum Mode {
    /// A subcommand for specify the mode to File, and "decrypt & decompress" by default or "compress & encrypt" by -c
    F(CAE),
    /// A subcommand for specify the mode to Dir, and "decrypt & decompress" by default or "compress & encrypt" by -c
    D(CAE),
}

#[derive(Parser)]
pub struct CAE {
    /// Specify the mode to "compress and encrypt"
    #[clap(short, long)]
    pub cae: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}