mod aes;
mod counter;

pub use self::aes::*;
pub use self::counter::*;

use std::collections::HashMap;

pub fn hex_to_bytes<T>(bs: T) -> Vec<u8>
where
    T: AsRef<[u8]>,
{
    data_encoding::HEXLOWER_PERMISSIVE
        .decode(bs.as_ref())
        .unwrap()
}

pub fn bytes_to_hex<T>(b: T) -> String
where
    T: AsRef<[u8]>,
{
    data_encoding::HEXLOWER.encode(b.as_ref())
}

pub fn xor_single<T>(b: &T, key: u8) -> impl Iterator<Item = u8> + '_
where
    T: AsRef<[u8]> + ?Sized,
{
    b.as_ref().iter().map(move |b| b ^ key)
}

pub fn xor_repeating<'a, T, U>(b: &'a T, key: &'a U) -> impl Iterator<Item = u8> + 'a
where
    T: AsRef<[u8]> + ?Sized,
    U: AsRef<[u8]> + ?Sized,
{
    b.as_ref()
        .iter()
        .zip(key.as_ref().iter().cycle())
        .map(|(b1, b2)| b1 ^ b2)
}

pub fn chisquare_frequency_score(input: &HashMap<&u8, usize>) -> f32 {
    let mut total = 0.0;

    for (&&b, &count) in input {
        let freq = count as f32 / input.len() as f32;
        let local_score = match b {
            b' ' => (freq - 0.18401).powi(2) / 0.18401,
            b'e' => (freq - 0.12702).powi(2) / 0.12702,
            b't' => (freq - 0.9056).powi(2) / 0.9056,
            b'a' => (freq - 0.8167).powi(2) / 0.8167,
            b'o' => (freq - 0.7507).powi(2) / 0.7507,
            b'i' => (freq - 0.6966).powi(2) / 0.6966,
            b'n' => (freq - 0.6749).powi(2) / 0.6749,
            b's' => (freq - 0.6327).powi(2) / 0.6327,
            b'h' => (freq - 0.6094).powi(2) / 0.6094,
            b'r' => (freq - 0.5987).powi(2) / 0.5987,
            b'd' => (freq - 0.4253).powi(2) / 0.4253,
            b'l' => (freq - 0.4025).powi(2) / 0.4025,
            b'c' => (freq - 0.2782).powi(2) / 0.2782,
            b'u' => (freq - 0.2758).powi(2) / 0.2758,
            b'm' => (freq - 0.2406).powi(2) / 0.2406,
            b'w' => (freq - 0.2360).powi(2) / 0.2360,
            b'f' => (freq - 0.2228).powi(2) / 0.2228,
            b'g' => (freq - 0.2015).powi(2) / 0.2015,
            b'y' => (freq - 0.1974).powi(2) / 0.1974,
            b'p' => (freq - 0.1929).powi(2) / 0.1929,
            b'b' => (freq - 0.1492).powi(2) / 0.1492,
            b'v' => (freq - 0.0978).powi(2) / 0.0978,
            b'k' => (freq - 0.0772).powi(2) / 0.0772,
            b'j' => (freq - 0.0153).powi(2) / 0.0153,
            b'x' => (freq - 0.0150).powi(2) / 0.0150,
            b'q' => (freq - 0.0095).powi(2) / 0.0095,
            b'z' => (freq - 0.0074).powi(2) / 0.0074,
            _ => (freq - 0.0).powi(2) / 0.0001,
        };

        total += local_score;
    }

    total
}

pub fn hamming_distance_byte(a: u8, b: u8) -> u32 {
    (a ^ b).count_ones()
}

pub fn hamming_distance<T, U>(a: T, b: U) -> usize
where
    T: AsRef<[u8]>,
    U: AsRef<[u8]>,
{
    assert_eq!(a.as_ref().len(), b.as_ref().len());
    a.as_ref()
        .iter()
        .zip(b.as_ref().iter())
        .map(|(&b1, &b2)| hamming_distance_byte(b1, b2) as usize)
        .sum()
}

pub fn crack_single_xor<T>(buffer: T) -> u8
where
    T: AsRef<[u8]>,
{
    (1..=255)
        .map(|key| {
            let deciphered: Vec<_> = xor_single(buffer.as_ref(), key)
                .map(|b| b.to_ascii_lowercase())
                .collect();
            let score = chisquare_frequency_score(&deciphered.counts());
            (score, key)
        })
        .min_by(|(score1, _), (score2, _)| score1.partial_cmp(score2).unwrap())
        .unwrap()
        .1
}

pub fn pkcs7_pad(buffer: &mut Vec<u8>, blocksize: usize) {
    let mut padding = blocksize - (buffer.len() % blocksize);
    if padding == 0 {
        padding = 16;
    }
    buffer.reserve_exact(padding);
    for _ in 0..padding {
        buffer.push(padding as u8);
    }
}
