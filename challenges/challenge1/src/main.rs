extern crate utils;
extern crate base64;

use base64::encode;

fn main() {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let bytes = utils::hex_to_bytes(s);
    assert_eq!(encode(&bytes), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}
