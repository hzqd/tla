use ades::cli::TimeUnit;
use aoko::{no_std::functions::ext::AnyExt1, standard::functions::fun::{measure_time_with_value, time_conversion_with_unit}};
use tla::{cli::get_args, compress_and_encrypt, decrypt_and_decompress};
use std::time::Duration;

fn tla() -> (impl FnOnce(Duration) -> u128, TimeUnit) {
    let (r#in, aes_key, des_key, ce, unit) = get_args().let_owned(|s| (s.input, s.aes_key, s.des_key, s.ce, s.time));
    if ce { compress_and_encrypt(&r#in, &aes_key, &des_key) }
    else { decrypt_and_decompress(&r#in, &aes_key, &des_key) }
    time_conversion_with_unit(unit)
}

fn main() {
    measure_time_with_value(tla)
        .let_owned(|((f, u), e)| println!("Execution time: {} {u:?}.", f(e)));
}