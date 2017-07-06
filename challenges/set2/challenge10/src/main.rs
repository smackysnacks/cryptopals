extern crate base64;
extern crate utils;

use base64::decode;

fn main() {
    let message = include_str!("../10.txt");
    let message = message.replace("\n", "");
    let message = message.replace("\r", "");
    let message = decode(&message).unwrap();
    let key = b"YELLOW SUBMARINE";
    let iv = [0; 16];

    let decrypted = utils::aes::aes_128_cbc_decrypt(key, &message, &iv);
    println!("{}", String::from_utf8_lossy(&decrypted));
}
