use cryptopals::utils::Counter;

pub fn solve() -> bool {
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = cryptopals::utils::hex_to_bytes(s);

    let mut best_score = std::f32::INFINITY;
    let mut best_key = 0;
    for key in 1..256u16 {
        let mut deciphered = cryptopals::utils::xor_single(&bytes, key as u8);
        deciphered = deciphered.iter().map(|b| b.to_ascii_lowercase()).collect();
        let n = cryptopals::utils::chisquare_frequency_score(&deciphered.as_slice().counts());
        if n < best_score {
            best_score = n;
            best_key = key as u8;
        }
    }

    String::from_utf8_lossy(&cryptopals::utils::xor_single(&bytes, best_key))
        == "Cooking MC's like a pound of bacon"
}
