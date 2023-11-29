use aoko::no_std::pipelines::tap::Tap;
use rayon::prelude::*;
use std::io::Write;
use xz2::write::{XzDecoder, XzEncoder};

pub fn mt_enc(data: &[u8]) -> Vec<u8> {
    data.par_chunks(data.len() / num_cpus::get() / 3)
        .flat_map(|grouped_data| {
            XzEncoder::new(vec![], 9)
                .tap_mut(|xz| xz.write_all(grouped_data).unwrap())
                .finish()
                .unwrap()
        })
        .collect()
}

pub fn mt_dec(data: &[u8]) -> Vec<u8> {
    XzDecoder::new_multi_decoder(vec![])
        .tap_mut(|xz| xz.write_all(data).unwrap())
        .finish()
        .unwrap()
}
