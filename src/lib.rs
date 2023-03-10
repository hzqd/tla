use std::fs;
use aoko::no_std::pipelines::{pipe::Pipe, tap::Tap};
use caesar::Caesar;
use multi::{mt_enc, mt_dec};
use rayon::{slice::ParallelSlice, prelude::ParallelIterator};
use tar::{Builder, Archive};
use ades::{Padding, aes_enc, aes_dec, des_enc, des_dec};

pub mod cli;
pub mod caesar;
pub mod multi;

fn padding<R>(aes: &str, des: &str, f: impl FnOnce(&[u8], &[u8]) -> R) -> R {
    aes.padding(32).as_bytes().pipe(|aes_key|
    des.padding(24).as_bytes().pipe(|des_key|
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

const GROUP: usize = 2 * 1024 * 1024 * 1024 - 1;

pub fn compress_and_encrypt(r#in: &str, aes: &str, des: &str) {
    Builder::new(vec![])
        .tap_mut(|b| b.append_dir_all("", r#in).unwrap_or_else(|_| b.append_path(r#in).unwrap()))
        .pipe(|b| b.into_inner().unwrap())
        .pipe_ref(|data| mt_enc(data))
        .cs_enc(175)
        .par_chunks(GROUP)
        .flat_map(|slice| padding(aes, des, |aes, des| aes_enc(aes)(slice).pipe(|ctx| des_enc(des)(&ctx))))
        .collect::<Vec<_>>()
        .tap_mut(|vec| mark_file_or_dir(r#in, vec)) // 0: dir, 1: file
        .pipe(|byt| fs::write(format!("{in}.tla"), byt.cs_enc(23)).unwrap())
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
    let mut read = fs::read(r#in).unwrap().cs_dec(23);
    let last = read.pop().unwrap();
    
    read.par_chunks(GROUP)
        .flat_map(|slice| padding(aes, des, |aes, des| des_dec(des)(slice).pipe(|ctx| aes_dec(aes)(&ctx))))
        .collect::<Vec<_>>()
        .cs_dec(175)
        .pipe_ref(|data| mt_dec(data))
        .pipe(|byt| r#in.trim_end_matches(".tla").pipe(|name|
            judge_file_or_dir(last, byt, |tar| tar.unpack(name).unwrap(),
                |tar| tar.entries().unwrap().for_each(|file| {
                    file.unwrap()
                        .unpack(name).unwrap();
                })
            )
        ))
}