use crypto::utils::{aes_128_cbc_encrypt, aes_128_ecb_encrypt, pkcs7_pad};
use rand::random;

#[derive(Eq, PartialEq)]
enum BlockMode {
    Ecb,
    Cbc,
}

fn random_data(min: u32, max: u32) -> Vec<u8> {
    (min..=max).map(|_| random::<u8>()).collect()
}

fn encrypt(input: &[u8]) -> (Vec<u8>, BlockMode) {
    let use_ecb = random::<bool>();
    let key = random::<[u8; 16]>();
    let prepend = random_data(5, 10);
    let append = random_data(5, 10);

    let mut data: Vec<u8> = prepend
        .into_iter()
        .chain(input.iter().cloned())
        .chain(append)
        .collect();
    pkcs7_pad(&mut data, 16);
    match use_ecb {
        true => (aes_128_ecb_encrypt(key, &data), BlockMode::Ecb),
        false => (aes_128_cbc_encrypt(key, &data, random()), BlockMode::Cbc),
    }
}

pub fn solve() -> bool {
    (0..50)
        .map(|_| {
            let (data, actual_mode) = encrypt(&[0; 64]);
            let chunks: Vec<&[u8]> = data.chunks(16).skip(1).take(2).collect();
            match chunks[0] == chunks[1] {
                true => (BlockMode::Ecb, actual_mode),
                false => (BlockMode::Cbc, actual_mode),
            }
        })
        .all(|(expected, actual)| expected == actual)
}
