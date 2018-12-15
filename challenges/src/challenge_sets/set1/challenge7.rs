use cryptopals::utils::aes_128_ecb_decrypt;

pub fn solve() -> bool {
    let message = include_str!("7.txt");
    let message = message.replace("\n", "");
    let message = message.replace("\r", "");
    let message = base64::decode(&message).unwrap();

    let key = b"YELLOW SUBMARINE";
    let result: Vec<u8> = message
        .chunks(16)
        .flat_map(|chunk| aes_128_ecb_decrypt(key, chunk))
        .collect();

    String::from_utf8_lossy(&result) == include_str!("7_solution.txt")
}
