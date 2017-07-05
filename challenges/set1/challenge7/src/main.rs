extern crate base64;
extern crate utils;

use base64::decode;

fn main() {
    let message = include_str!("../7.txt");
    let message = message.replace("\n", "");
    let message = message.replace("\r", "");
    let message = decode(&message).unwrap();

    let key = b"YELLOW SUBMARINE";
    let result = utils::aes::aes_128_ecb_decrypt(&key[..], &message);
    println!("{}", String::from_utf8_lossy(&result));
}
