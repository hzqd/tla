use aoko::{standard::functions::fun::{measure_time_with_value, time_conversion_with_unit}, no_std::{pipelines::pipe::Pipe, algebraic::sum::TimeUnit}};
use tla::{cli::{get_args, Compress::{C, N, D}}, compress::{mt_enc, mt_dec}, encrypt::{Caesar, Crypto}, pack::{archive, restore}};
use std::time::Duration;

fn tla() -> (impl FnOnce(Duration) -> f32, TimeUnit) {
    //let (r#in, aes_key, des_key, compress, unit) = get_args().pipe(|s| (s.input, s.aes_key, s.des_key, s.compress, s.time));
    let args = get_args();
    let pack = |f: fn(Vec<u8>) -> Vec<u8>| archive(&args.input, |data| f(data).cs_enc(args.caesar).encrypt(&args.aes_key, &args.des_key, &args.rsa_private_key));
    let unpack = |f: fn(Vec<u8>) -> Vec<u8>| restore(&args.input, |data| data.decrypt(&args.aes_key, &args.des_key, &args.rsa_private_key).cs_dec(args.caesar).pipe(f));

    match args.compress {
        C => pack(|this| mt_enc(&this)),
        D => unpack(|this| mt_dec(&this)),
        N(t) => if t.tar { pack(|this| this) } else { unpack(|this| this) },
    }
    time_conversion_with_unit(args.time)
}

fn main() {
    measure_time_with_value(tla)
        .pipe(|(e, (f, u))| println!("Execution time: {} {u:?}.", f(e)));
}