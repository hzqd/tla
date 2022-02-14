use std::{fs, io::Read};
use aoko::no_std::functions::ext::AnyExt1;
use xz2::read::{XzEncoder, XzDecoder};
use tar::{Builder, Archive};
use ades::{Padding, aes_enc, aes_dec, des_enc, des_dec};

pub mod cli;

fn padding<R>(aes: &str, des: &str, f: impl FnOnce(&[u8], &[u8]) -> R) -> R {
    aes.padding(32).as_bytes().let_owned(|aes_key|
    des.padding(24).as_bytes().let_owned(|des_key|
        f(aes_key, des_key)
    ))
}

fn mark_file_or_dir(r#in: &str, vec: &mut Vec<u8>) {
    let meta = fs::metadata(r#in).unwrap();
    if meta.is_dir() {
        vec.push(0)
    } else if meta.is_file() {
        vec.push(1)
    } else {
        panic!("unsupported file type")
    }
}

pub fn compress_and_encrypt(r#in: &str, aes: &str, des: &str) {
    let compressed = Builder::new(vec![])
        .also_mut(|b| b.append_dir_all("", r#in).unwrap_or_else(|_| b.append_path(r#in).unwrap()))
        .let_owned(|b| (vec![], b.into_inner().unwrap()))
        .also_mut(|(vec, data)| XzEncoder::new(data.as_slice(), 9).read_to_end(vec).unwrap()).0;

    padding(aes, des, |aes, des|
        aes_enc(aes)(&compressed)
            .let_owned(|ctx| des_enc(des)(&ctx))
            .also_mut(|vec| mark_file_or_dir(r#in, vec)) // 0: dir, 1: file
            .let_owned(|byt| fs::write(format!("{in}.tla"), byt).unwrap())
    )
}

fn judge_file_or_dir(last: u8, byt: Vec<u8>, f1: impl FnOnce(&mut Archive<&[u8]>), f2: impl FnOnce(&mut Archive<&[u8]>)) {
    let tar = &mut Archive::new(byt.as_slice());
    match last {
        0 => f1(tar),
        1 => f2(tar),
        _ => panic!("file type error")
    }
}

pub fn decrypt_and_decompress(r#in: &str, aes: &str, des: &str) {
    let mut read = fs::read(r#in).unwrap();
    let last = read.pop().unwrap();
    
    padding(aes, des, |aes, des|
        des_dec(des)(&read)
            .let_owned(|ctx| (vec![], aes_dec(aes)(&ctx)))
    )
    .also_mut(|(vec, data)| XzDecoder::new(data.as_slice()).read_to_end(vec).unwrap()).0
    .let_owned(|byt| r#in.trim_end_matches(".tla").let_owned(|name| {
        judge_file_or_dir(last, byt, |tar| tar.unpack(name).unwrap(),
            |tar| tar.entries().unwrap().for_each(|file| {
                file.unwrap()
                    .unpack(name).unwrap();
            })
        )
    }))
}