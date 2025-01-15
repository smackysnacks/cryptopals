use base64::prelude::*;

pub fn solve() -> bool {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = crypto::utils::hex_to_bytes(s);
    BASE64_STANDARD.encode(&bytes)
        == "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
}
