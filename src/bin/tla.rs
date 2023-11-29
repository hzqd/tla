use aoko::{
    no_std::{algebraic::sum::TimeUnit, pipelines::pipe::Pipe},
    standard::functions::fun::{measure_time_with_value, time_conversion_with_unit},
};
use std::time::Duration;
use tla::{
    cli::{
        get_args,
        Compress::{C, D, N},
    },
    compress::{mt_dec, mt_enc},
    encrypt::{Caesar, Crypto},
    pack::{archive, restore},
};

fn tla() -> (impl FnOnce(Duration) -> f32, TimeUnit) {
    let args = get_args();
    let pack = |f: fn(Vec<u8>) -> Vec<u8>| {
        archive(&args.input, |data| {
            f(data)
                .cs_enc(args.caesar)
                .encrypt(&args.aes_key, &args.des_key, &args.rsa_private_key)
        })
    };
    let unpack = |f: fn(Vec<u8>) -> Vec<u8>| {
        restore(&args.input, |data| {
            data.decrypt(&args.aes_key, &args.des_key, &args.rsa_private_key)
                .cs_dec(args.caesar)
                .pipe(f)
        })
    };

    match args.compress {
        C => pack(|this| mt_enc(&this)),
        D => unpack(|this| mt_dec(&this)),
        N(t) if t.tar => pack(|this| this),
        _ => unpack(|this| this),
    }
    time_conversion_with_unit(args.time)
}

fn main() {
    measure_time_with_value(tla).pipe(|(e, (f, u))| println!("Execution time: {} {u:?}.", f(e)));
}
