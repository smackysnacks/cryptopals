use crypto::utils::aes_128_ecb_decrypt;

pub fn solve() -> bool {
    let message = include_str!("7.txt");
    let message = message.replace("\n", "");
    let message = message.replace("\r", "");
    let message = base64::decode(&message).unwrap();

    let key = b"YELLOW SUBMARINE";
    let result = aes_128_ecb_decrypt(*key, &message);

    String::from_utf8_lossy(&result) == include_str!("7_solution.txt")
}
