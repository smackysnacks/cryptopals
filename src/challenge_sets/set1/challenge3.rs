use crypto::utils::Counter;

pub fn solve() -> bool {
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = crypto::utils::hex_to_bytes(s);

    let mut best_score = f32::INFINITY;
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

    String::from_utf8_lossy(&crypto::utils::xor_single(&bytes, best_key).collect::<Vec<u8>>())
        == "Cooking MC's like a pound of bacon"
}
