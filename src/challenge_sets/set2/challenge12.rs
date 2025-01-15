use crypto::utils::{aes_128_ecb_encrypt, pkcs7_pad};

use std::sync::LazyLock;

use base64::prelude::*;
use rand::random;

static KEY: LazyLock<[u8; 16]> = LazyLock::new(random);

#[derive(Debug, Eq, PartialEq)]
enum BlockMode {
    Ecb,
    Cbc,
}

fn detect_block_cipher_mode(block_size: usize) -> BlockMode {
    let input = b"A".repeat(2 * block_size);
    let encrypted = encrypt(&input);
    let v: Vec<&[u8]> = encrypted.chunks(block_size).take(2).collect();

    match v[0] == v[1] {
        true => BlockMode::Ecb,
        false => BlockMode::Cbc,
    }
}

fn encrypt(input: &[u8]) -> Vec<u8> {
    let secret = b"Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let secret = BASE64_STANDARD.decode(secret).unwrap();
    let mut data: Vec<u8> = input.iter().cloned().chain(secret).collect();
    pkcs7_pad(&mut data, 16);
    aes_128_ecb_encrypt(*KEY, &data)
}

fn detect_block_size() -> usize {
    let initial_len = encrypt(&[]).len();
    (1..)
        .map(|i| encrypt(&b"A".repeat(i)).len())
        .find(|&new_length| new_length != initial_len)
        .map_or(0, |new_length| new_length - initial_len)
}

fn compute_secret_length() -> usize {
    let initial_len = encrypt(&[]).len();
    (1..)
        .map(|i| (i, encrypt(&b"A".repeat(i)).len()))
        .find(|&(_, new_length)| new_length != initial_len)
        .map_or(0, |(i, _)| initial_len - i)
}

pub fn solve() -> bool {
    let block_size = detect_block_size();
    let secret_length = compute_secret_length();
    assert_eq!(BlockMode::Ecb, detect_block_cipher_mode(block_size));

    let secret_with_block_length = secret_length + (block_size - (secret_length % block_size));
    let mut cracked = vec![b'a'; 2 * secret_with_block_length];
    let mut i = 1;
    while i < secret_length {
        let start = i;
        let end = secret_with_block_length;
        let unknown = encrypt(&cracked[start..end]);

        // now call encrypt for [0..255] until we match the unknown byte
        (0..=255)
            .map(|b| {
                cracked[end + i - 1] = b;
                let to_encrypt = &cracked[start..end + i];
                encrypt(to_encrypt)
            })
            .find(|v| v[0..secret_with_block_length] == unknown[0..secret_with_block_length]);

        i += 1;
    }

    let solution = String::from_utf8_lossy(
        &cracked[i + (secret_with_block_length - secret_length)..i + secret_with_block_length - 1],
    );

    solution.contains("standby waving")
}
