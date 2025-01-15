use base64::prelude::*;

pub fn solve() -> bool {
    let message = include_str!("10.txt");
    let message = message.replace("\n", "");
    let message = message.replace("\r", "");
    let message = BASE64_STANDARD.decode(&message).unwrap();
    let key = b"YELLOW SUBMARINE";
    let iv = [0; 16];

    let decrypted = crypto::utils::aes_128_cbc_decrypt(*key, &message, iv);
    include_str!("10_solution.txt") == String::from_utf8_lossy(&decrypted)
}
