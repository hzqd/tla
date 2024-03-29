use aoko::{no_std::pipelines::pipe::Pipe, standard::functions::fun::measure_time};
use rsa::RsaPrivateKey;
use std::fs;

const BIT_SIZE: usize = 1024 * 5 + 1024 / 2;

fn main() {
    let gen = || {
        RsaPrivateKey::new(&mut rand::thread_rng(), BIT_SIZE)
            .unwrap()
            .pipe_ref(serde_json::to_string)
            .unwrap()
            .pipe(|s| fs::write("tla-rsa", s))
            .unwrap()
    };

    measure_time(gen)
        .as_secs_f32()
        .pipe(|e| println!("Execution time: {e} Secs."));
}
