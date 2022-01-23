use ades::cli::TimeUnit;
use aoko::{no_std::functions::ext::AnyExt1, standard::functions::fun::{measure_time_with_value, time_conversion_with_unit}};
use tla::{cli::{CompressAndEncrypt::M, get_args}, compress_and_encrypt, decrypt_and_decompress};
use std::time::Duration;

fn tla() -> (impl FnOnce(Duration) -> u128, TimeUnit) {
    let (r#in, out, aes_key, des_key, unit, subcmd) = get_args().let_owned(|s| (s.input, s.output, s.aes_key, s.des_key, s.time, s.subcmd));
    let M(mode) = subcmd;
    match mode.compress_and_encrypt {
        true => compress_and_encrypt(r#in, out, aes_key, des_key),
        false => decrypt_and_decompress(r#in, out, aes_key, des_key),
    }
    time_conversion_with_unit(unit)
}

fn main() {
    measure_time_with_value(tla)
        .let_owned(|((f, u), e)| println!("Execution time: {} {:?}.", f(e), u));
}