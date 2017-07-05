extern crate openssl;

use self::openssl::symm::{decrypt, encrypt, Cipher};

pub fn aes_128_ecb_encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    encrypt(cipher, key, None, data).unwrap()
}

pub fn aes_128_ecb_decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    decrypt(cipher, key, None, data).unwrap()
}
