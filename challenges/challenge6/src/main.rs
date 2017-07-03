extern crate base64;
extern crate utils;

use base64::decode;

fn best_keysize(buffer: &[u8]) -> usize {
    // calculate the normalized edit distances
    // Vec<(score, keysize)>
    let mut scores: Vec<(f32, usize)> = Vec::new();
    for keysize in 2..41 {
        let mut distance = 0;
        for i in 0..buffer.len()/keysize-1 {
            distance += utils::hamming_distance(&buffer[keysize*i..keysize*(i+1)],
                                                &buffer[keysize*(i+1)..keysize*(i+2)]);
        }
        let distance = distance as f32 / (buffer.len() / keysize) as f32;
        scores.push((distance / keysize as f32, keysize));
    }
    scores.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    scores[0].1
}

fn crack(blocks: &Vec<&[u8]>) -> Vec<u8> {
    let mut stitched = Vec::new();
    let mut key = Vec::new();

    for i in 0..blocks[0].len() {
        for j in 0..blocks.len() {
            if i < blocks[j].len() {
                stitched.push(blocks[j][i]);
            }
        }
        key.push(utils::crack_single_xor(&stitched));
        stitched.clear();
    }

    key
}

fn main() {
    let message = include_str!("../6.txt");
    let message = message.replace("\n", "");
    let message = decode(&message[..]).unwrap();

    let keysize = best_keysize(&message);
    let blocks = message.chunks(keysize).collect();
    let key = crack(&blocks);
    let decrypted = utils::xor_repeating(&message, &key);

    println!("key {}", String::from_utf8_lossy(&key));
    println!("{}", String::from_utf8_lossy(&decrypted));
}
