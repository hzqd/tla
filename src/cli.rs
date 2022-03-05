use ades::cli::TimeUnit;
use clap::Parser;

/// A CLI tool for compressing and encrypting a file or directory. (tar, lzma, aes & des)

#[derive(Parser)]
#[clap(version = "0.1.4", author = "hzqd <hzqelf@yeah.net>")]
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

    /// Use it to "compress & encrypt", Omit it to "decrypt & decompress"
    #[clap(short, long)]
    pub ce: bool,

    /// Specify the time unit, support nanos, micros, millis, secs
    #[clap(short, long, default_value = "millis")]
    pub time: TimeUnit,
}

pub fn get_args() -> Args {
    Args::parse()
}