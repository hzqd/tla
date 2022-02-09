use ades::cli::TimeUnit;
use aoko::{no_std::functions::ext::AnyExt1, standard::functions::fun::{measure_time_with_value, time_conversion_with_unit}};
use tla::{cli::{Mode::*, get_args}, compress_and_encrypt, file_dad, dir_dad};
use std::time::Duration;

fn tla() -> (impl FnOnce(Duration) -> u128, TimeUnit) {
    let (r#in, out, aes_key, des_key, f_or_d, unit) = get_args().let_owned(|s| (s.input, s.output, s.aes_key, s.des_key, s.subcmd, s.time));
    match f_or_d {
        F(bool) | D(bool) if bool.cae => compress_and_encrypt(&r#in, &out, &aes_key, &des_key),
        F(_) => file_dad(&r#in, &out, &aes_key, &des_key),
        D(_) => dir_dad(&r#in, &out, &aes_key, &des_key),
    }
    time_conversion_with_unit(unit)
}

fn main() {
    measure_time_with_value(tla)
        .let_owned(|((f, u), e)| println!("Execution time: {} {u:?}.", f(e)));
}