use crypto::utils::Counter;
use rayon::prelude::*;

struct Score {
    key: u8,
    score: f32,
}

fn rank(s: &str) -> Score {
    let bytes = crypto::utils::hex_to_bytes(s);

    let mut best_score = std::f32::INFINITY;
    let mut best_key = 0;
    for key in 1..=255 {
        let deciphered: Vec<_> = crypto::utils::xor_single(&bytes, key)
            .map(|b| b.to_ascii_lowercase())
            .collect();
        let n = crypto::utils::chisquare_frequency_score(&deciphered.as_slice().counts());
        if n < best_score {
            best_score = n;
            best_key = key;
        }
    }

    Score {
        key: best_key,
        score: best_score,
    }
}

pub fn solve() -> bool {
    let s = include_str!("4.txt");
    let ranked: Vec<_> = s.par_lines().map(|l| rank(l)).collect();

    let mut max = 0;
    for i in 0..ranked.len() {
        if ranked[i].score < ranked[max].score {
            max = i;
        }
    }

    let bytes = crypto::utils::hex_to_bytes(s.lines().nth(max).unwrap());
    "Now that the party is jumping\n"
        == String::from_utf8_lossy(
            &crypto::utils::xor_single(&bytes, ranked[max].key).collect::<Vec<u8>>(),
        )
}
