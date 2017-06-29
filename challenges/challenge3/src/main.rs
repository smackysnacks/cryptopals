extern crate utils;

use utils::counter::Counter;

use std::ascii::AsciiExt;
use std::collections::HashMap;

fn xor_single(buffer: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut outbuffer = Vec::with_capacity(buffer.len());
    for b in buffer {
        outbuffer.push(b ^ key);
    }
    outbuffer
}

// score using chi-squared test
fn score(input: &HashMap<&u8, usize>) -> f32 {
    let mut total = 0.0;

    for (&&b, &count) in input {
        let freq = count as f32 / input.len() as f32;
        let local_score = match b {
            b' ' => (freq - 0.18401).abs().powi(2) / 0.18401,
            b'e' => (freq - 0.12702).abs().powi(2) / 0.12702,
            b't' => (freq - 0.9056).abs().powi(2) / 0.9056,
            b'a' => (freq - 0.8167).abs().powi(2) / 0.8167,
            b'o' => (freq - 0.7507).abs().powi(2) / 0.7507,
            b'i' => (freq - 0.6966).abs().powi(2) / 0.6966,
            b'n' => (freq - 0.6749).abs().powi(2) / 0.6749,
            b's' => (freq - 0.6327).abs().powi(2) / 0.6327,
            b'h' => (freq - 0.6094).abs().powi(2) / 0.6094,
            b'r' => (freq - 0.5987).abs().powi(2) / 0.5987,
            b'd' => (freq - 0.4253).abs().powi(2) / 0.4253,
            b'l' => (freq - 0.4025).abs().powi(2) / 0.4025,
            b'c' => (freq - 0.2782).abs().powi(2) / 0.2782,
            b'u' => (freq - 0.2758).abs().powi(2) / 0.2758,
            b'm' => (freq - 0.2406).abs().powi(2) / 0.2406,
            b'w' => (freq - 0.2360).abs().powi(2) / 0.2360,
            b'f' => (freq - 0.2228).abs().powi(2) / 0.2228,
            b'g' => (freq - 0.2015).abs().powi(2) / 0.2015,
            b'y' => (freq - 0.1974).abs().powi(2) / 0.1974,
            b'p' => (freq - 0.1929).abs().powi(2) / 0.1929,
            b'b' => (freq - 0.1492).abs().powi(2) / 0.1492,
            b'v' => (freq - 0.0978).abs().powi(2) / 0.0978,
            b'k' => (freq - 0.0772).abs().powi(2) / 0.0772,
            b'j' => (freq - 0.0153).abs().powi(2) / 0.0153,
            b'x' => (freq - 0.0150).abs().powi(2) / 0.0150,
            b'q' => (freq - 0.0095).abs().powi(2) / 0.0095,
            b'z' => (freq - 0.0074).abs().powi(2) / 0.0074,
            _    => (freq - 0.0).abs().powi(2) / 0.0001
        };

        total += local_score;
    }

    total
}

fn main() {
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = utils::hex_to_bytes(s);

    let mut best_score = std::f32::INFINITY;
    let mut best_key = 0;
    for key in 1..256u16 {
        let mut deciphered = xor_single(&bytes, key as u8);
        deciphered = deciphered.iter().map(|b| b.to_ascii_lowercase()).collect();
        let n = score(&deciphered.as_slice().counts());
        if n < best_score {
            best_score = n;
            best_key = key as u8;
        }
    }

    println!("message using {} as key:", best_key);
    println!("{}", String::from_utf8_lossy(&xor_single(&bytes, best_key)));
}
