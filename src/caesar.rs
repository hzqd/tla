use aoko::standard::parallelisms::par_vec_ext::ParMutExt;

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

#[test]
fn caesar_encrypt() {
    assert_eq!(vec![4, 5, 6], vec![1, 2, 3].cs_enc(3));
    assert_eq!(vec![0, 1, 2], vec![253, 254, 255].cs_enc(3));
}

#[test]
fn caesar_decrypt() {
    assert_eq!(vec![1, 2, 3], vec![4, 5, 6].cs_dec(3));
    assert_eq!(vec![253, 254, 255], vec![0, 1, 2].cs_dec(3));
}