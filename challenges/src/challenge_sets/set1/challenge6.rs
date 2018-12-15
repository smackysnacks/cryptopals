fn best_keysize(buffer: &[u8]) -> usize {
    // calculate the normalized edit distances
    // Vec<(score, keysize)>
    let mut scores: Vec<(f32, usize)> = Vec::new();
    for keysize in 2..41 {
        let mut distance = 0;
        for i in 0..buffer.len() / keysize - 1 {
            distance += cryptopals::utils::hamming_distance(
                &buffer[keysize * i..keysize * (i + 1)],
                &buffer[keysize * (i + 1)..keysize * (i + 2)],
            );
        }
        let distance = distance as f32 / (buffer.len() / keysize) as f32;
        scores.push((distance / keysize as f32, keysize));
    }
    scores.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    scores[0].1
}

fn crack(blocks: &[&[u8]]) -> Vec<u8> {
    let mut stitched = Vec::new();
    let mut key = Vec::new();

    for i in 0..blocks[0].len() {
        for j in 0..blocks.len() {
            if i < blocks[j].len() {
                stitched.push(blocks[j][i]);
            }
        }
        key.push(cryptopals::utils::crack_single_xor(&stitched));
        stitched.clear();
    }

    key
}

pub fn solve() -> bool {
    let message = include_str!("6.txt");
    let message = message.replace("\n", "");
    let message = base64::decode(&message[..]).unwrap();

    let keysize = best_keysize(&message);
    let blocks: Vec<_> = message.chunks(keysize).collect();
    let key = crack(&blocks);
    let decrypted = cryptopals::utils::xor_repeating(&message, &key);

    String::from_utf8_lossy(&decrypted) == include_str!("6_solution.txt")
}
