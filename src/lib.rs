use std::{fs, io::Read};
use aoko::no_std::functions::ext::AnyExt1;
use xz2::read::{XzEncoder, XzDecoder};
use tar::{Builder, Archive};
use ades::{Padding, aes_enc, aes_dec, des_enc, des_dec};

pub mod cli;

pub fn compress_and_encrypt(r#in: String, out: String, aes: String, des: String) {
    let compressed = Builder::new(vec![])
        .also_mut(|b| b.append_dir_all("", r#in).unwrap())
        .let_owned(|b| (vec![], b.into_inner().unwrap()))
        .also_mut(|(vec, data)| XzEncoder::new(data.as_slice(), 9).read_to_end(vec).unwrap()).0;

    aes.padding(32).as_bytes().let_owned(|aes_key|
    des.padding(24).as_bytes().let_owned(|des_key|
        aes_enc(aes_key)(&compressed)
            .let_owned(|ctx| des_enc(des_key)(&ctx))
            .let_owned(|byt| fs::write(out, byt).unwrap())
    ))
}

pub fn decrypt_and_decompress(r#in: String, out: String, aes: String, des: String) {
    aes.padding(32).as_bytes().let_owned(|aes_key|
    des.padding(24).as_bytes().let_owned(|des_key|
        des_dec(des_key)(&fs::read(r#in).unwrap())
            .let_owned(|ctx| (vec![], aes_dec(aes_key)(&ctx)))
    ))
    .also_mut(|(vec, data)| XzDecoder::new(data.as_slice()).read_to_end(vec).unwrap()).0
    .let_owned(|byt| Archive::new(byt.as_slice()).unpack(out).unwrap());
}