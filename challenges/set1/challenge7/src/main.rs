extern crate base64;
extern crate openssl;
extern crate utils;

use base64::decode;
use openssl::symm::{decrypt, Cipher};

fn main() {
    let message = include_str!("../7.txt");
    let message = message.replace("\n", "");
    let message = message.replace("\r", "");
    let message = decode(&message).unwrap();

    let key = b"YELLOW SUBMARINE";
    let cipher = Cipher::aes_128_ecb();
    let result = decrypt(cipher, &key[..], None, &message);
    println!("{}", String::from_utf8_lossy(&result.unwrap()));
}
