use crate::utils::xor_repeating;
use aes::Aes128;
use block_modes::{block_padding::NoPadding, BlockMode, Ecb};
use std::convert::TryInto;

pub fn aes_128_ecb_encrypt(key: [u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Ecb::<Aes128, NoPadding>::new_from_slices(&key, &[]).unwrap();
    cipher.encrypt_vec(data)
}

pub fn aes_128_ecb_decrypt(key: [u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Ecb::<Aes128, NoPadding>::new_from_slices(&key, &[]).unwrap();
    cipher.decrypt_vec(data).unwrap()
}

pub fn aes_128_cbc_encrypt(key: [u8; 16], data: &[u8], iv: [u8; 16]) -> Vec<u8> {
    data.chunks(16)
        .fold((Vec::with_capacity(data.len()), iv), |(mut v, c), chunk| {
            let p: Vec<u8> = xor_repeating(chunk, &c).collect();
            let encrypted_block = aes_128_ecb_encrypt(key, &p);
            v.extend(encrypted_block.iter());

            (v, encrypted_block.try_into().unwrap())
        })
        .0
}

pub fn aes_128_cbc_decrypt(key: [u8; 16], data: &[u8], iv: [u8; 16]) -> Vec<u8> {
    let mut dec_buf: Vec<u8> = Vec::with_capacity(data.len());
    let mut c = Vec::with_capacity(16);
    c.extend(iv.iter());

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

#[cfg(test)]
mod tests {
    use crate::utils::{aes_128_cbc_decrypt, aes_128_cbc_encrypt};
    use rand::random;

    #[test]
    fn test_aes_128_cbc_crypt() {
        let key = random::<[u8; 16]>();
        let iv = random::<[u8; 16]>();
        let input: Vec<u8> = (0..256).map(|_| random::<u8>()).collect();

        let encrypted = aes_128_cbc_encrypt(key, &input, iv);
        let decrypted = aes_128_cbc_decrypt(key, &encrypted, iv);

        assert_eq!(input, decrypted);
    }
}
