extern crate utils;

use utils::counter::Counter;
use std::ascii::AsciiExt;

struct Score {
    key: u8,
    score: f32
}

fn rank(s: &str) -> Score {
    let bytes = utils::hex_to_bytes(s);

    let mut best_score = std::f32::INFINITY;
    let mut best_key = 0;
    for key in 1..256u16 {
        let mut deciphered = utils::xor_single(&bytes, key as u8);
        deciphered = deciphered.iter().map(|b| b.to_ascii_lowercase()).collect();
        let n = utils::chisquare_frequency_score(&deciphered.as_slice().counts());
        if n < best_score {
            best_score = n;
            best_key = key as u8;
        }
    }

    Score { key: best_key, score: best_score }
}

fn main() {
    let s = include_str!("../4.txt");
    let ranked: Vec<_> = s.lines().map(|l| rank(l)).collect();

    let mut max = 0;
    for i in 0..ranked.len() {
        if ranked[i].score < ranked[max].score {
            max = i;
        }
    }

    println!("{} xor {}:", s.lines().nth(max).unwrap(), ranked[max].key);
    let bytes = utils::hex_to_bytes(s.lines().nth(max).unwrap());
    println!("{}", String::from_utf8_lossy(&utils::xor_single(&bytes, ranked[max].key)));
}
