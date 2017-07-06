extern crate openssl;

use self::openssl::symm::{encrypt, Cipher, Crypter, Mode};

pub fn aes_128_ecb_encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    encrypt(cipher, key, None, data).unwrap()
}

pub fn aes_128_ecb_decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, None).unwrap();
    let mut output = Vec::with_capacity(data.len());
    output.resize(data.len()+16, 0);

    crypter.pad(false);
    crypter.update(data, &mut output).unwrap();
    crypter.finalize(&mut output).unwrap();

    output.truncate(16);
    output
}

pub fn aes_128_cbc_decrypt(key: &[u8], data: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut dec_buf: Vec<u8> = Vec::with_capacity(data.len());
    let mut c = Vec::with_capacity(16);
    c.extend(iv);

    let mut i = 0;
    while i * 16 < data.len() {
        if i * 16 + 16 > data.len() {
            let block = ::xor_repeating(&aes_128_ecb_decrypt(key, &data[i*16..]), &c);
            dec_buf.extend(&block);
            c.clear();
            c.extend(&data[i*16..]);
        } else {
            let block = ::xor_repeating(&aes_128_ecb_decrypt(key, &data[i*16..i*16+16]), &c);
            dec_buf.extend(&block);
            c.clear();
            c.extend(&data[i*16..i*16+16]);
        }

        i += 1;
    }

    dec_buf
}
