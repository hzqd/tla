use aoko::no_std::pipelines::tap::Tap;
use rayon::prelude::*;
use std::io::Write;
use xz2::write::{XzEncoder, XzDecoder};

pub fn mt_enc(data: &[u8]) -> Vec<u8> {
    let data = data.par_chunks(data.len() / num_cpus::get() / 3).collect::<Vec<_>>();

    let mut multi_xz = (0..data.len())
        .into_par_iter()
        .map(|_| XzEncoder::new(vec![], 9))
        .collect::<Vec<_>>();

    multi_xz.par_iter_mut()
        .enumerate()    // SAFETY: last index == data.len() - 1
        .for_each(|(index, xz)| xz.write_all(unsafe { data.get_unchecked(index) }).unwrap());

    multi_xz.into_par_iter().flat_map(|xz| xz.finish().unwrap()).collect()
}

pub fn mt_dec(data: &[u8]) -> Vec<u8> {
    XzDecoder::new_multi_decoder(vec![])
        .tap_mut(|xz| xz.write_all(data).unwrap())
        .finish().unwrap()
}