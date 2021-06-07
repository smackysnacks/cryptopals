use aes::Aes128;
use block_modes::{block_padding::NoPadding, BlockMode, Ecb};

pub fn aes_128_ecb_encrypt(key: [u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Ecb::<Aes128, NoPadding>::new_from_slices(&key, &[]).unwrap();
    cipher.encrypt_vec(data)
}

pub fn aes_128_ecb_decrypt(key: [u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Ecb::<Aes128, NoPadding>::new_from_slices(&key, &[]).unwrap();
    cipher.decrypt_vec(data).unwrap()
}

pub fn aes_128_cbc_decrypt(key: [u8; 16], data: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut dec_buf: Vec<u8> = Vec::with_capacity(data.len());
    let mut c = Vec::with_capacity(16);
    c.extend(iv);

    let mut i = 0;
    while i * 16 < data.len() {
        if i * 16 + 16 > data.len() {
            dec_buf.extend(super::xor_repeating(
                &aes_128_ecb_decrypt(key, &data[i * 16..]),
                &c,
            ));
            c.clear();
            c.extend(&data[i * 16..]);
        } else {
            dec_buf.extend(super::xor_repeating(
                &aes_128_ecb_decrypt(key, &data[i * 16..i * 16 + 16]),
                &c,
            ));
            c.clear();
            c.extend(&data[i * 16..i * 16 + 16]);
        }

        i += 1;
    }

    dec_buf
}
