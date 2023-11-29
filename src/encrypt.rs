use std::fs;
use ades::{des_enc as des_encrypt, des_dec as des_decrypt, aes_enc as aes_encrypt, aes_dec as aes_decrypt, Padding};
use aoko::{standard::parallelisms::par_vec_ext::ParMutExt, no_std::pipelines::pipe::Pipe};
use rayon::{slice::ParallelSlice, prelude::ParallelIterator};
use rsa::{RsaPrivateKey, Pkcs1v15Encrypt};

pub trait Caesar {
    fn cs_enc(self, n: u8) -> Self;
    fn cs_dec(self, n: u8) -> Self;
}

impl Caesar for Vec<u8> {
    fn cs_enc(self, n: u8) -> Self {
        self.on_each(|u8| *u8 = u8.wrapping_add(n))
    }
    fn cs_dec(self, n: u8) -> Self {
        self.on_each(|u8| *u8 = u8.wrapping_sub(n))
    }
}

const GROUP: usize = 2 * 1024 * 1024 * 1024 - 1;

fn crypto<'a, F>(padded: &'a str, r#in: &[u8], f: impl FnOnce(&'a [u8]) -> F) -> Vec<u8> where F: Sync + Send + Fn(&[u8]) -> Vec<u8> {
    padded.as_bytes().pipe(f).pipe(|any_crypt| r#in.par_chunks(GROUP).flat_map(any_crypt)).collect()
}

macro_rules! crypt {
    ($key:expr, $pad:expr, $in:expr, $fn:expr) => {
        {
            let padded = &$key.padding($pad);
            crypto(padded, $in, $fn)
        }
    };
}

macro_rules! gen_crypt {
    ($($name:ident $fn:ident $pad:expr)*) => {
        trait Crypt { $( fn $name(self, key: &str) -> Vec<u8>; )* }
        impl Crypt for &[u8] { $( fn $name(self, key: &str) -> Vec<u8> { crypt!(key, $pad, self, $fn) } )* }
    };
}

gen_crypt! {
    des_enc des_encrypt 24      des_dec des_decrypt 24
    aes_enc aes_encrypt 32      aes_dec aes_decrypt 32
}

fn get_rsa_key(path: &str) -> RsaPrivateKey {
    fs::read(path).unwrap().as_slice()
        .pipe(serde_json::from_slice).unwrap()
}

pub trait RSA {
    fn rsa_enc(self, path: &str) -> Vec<u8>;
    fn rsa_dec(self, path: &str) -> Vec<u8>;
}

impl RSA for &[u8] {
    fn rsa_enc(self, path: &str) -> Vec<u8> {
        get_rsa_key(path).to_public_key().encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, self).unwrap()
    }

    fn rsa_dec(self, path: &str) -> Vec<u8> {
        get_rsa_key(path).decrypt(Pkcs1v15Encrypt, self).unwrap()
    }
}

pub trait Crypto {
    fn encrypt(self, aes: &str, des: &str, rsa: &str) -> Vec<u8>;
    fn decrypt(self, aes: &str, des: &str, rsa: &str) -> Vec<u8>;
}

impl Crypto for Vec<u8> {
    fn encrypt(self, aes: &str, des: &str, rsa: &str) -> Vec<u8> {
        match (des, aes, rsa) {
            ("", "", "") => self,
            (_, "", "") => self.des_enc(des),
            ("", _, "") => self.aes_enc(aes),
            ("", "", _) => self.rsa_enc(rsa),
            (_, _, "") => self.des_enc(des).aes_enc(aes),
            (_, "", _) => self.des_enc(des).rsa_enc(rsa),
            ("", _, _) => self.aes_enc(aes).rsa_enc(rsa),
            _ => self.des_enc(des).aes_enc(aes).rsa_enc(rsa),
        }
    }

    fn decrypt(self, aes: &str, des: &str, rsa: &str) -> Vec<u8> {
        match (des, aes, rsa) {
            ("", "", "") => self,
            (_, "", "") => self.des_dec(des),
            ("", _, "") => self.aes_dec(aes),
            ("", "", _) => self.rsa_dec(rsa),
            (_, _, "") => self.aes_dec(aes).des_dec(des),
            (_, "", _) => self.rsa_dec(rsa).des_dec(des),
            ("", _, _) => self.rsa_dec(rsa).aes_dec(aes),
            _ => self.rsa_dec(rsa).aes_dec(aes).des_dec(des),
        }
    }
}
