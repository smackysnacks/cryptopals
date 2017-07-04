extern crate utils;

fn main() {
    let message = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let key = b"ICE";

    let e = utils::xor_repeating(&message[..], &key[..]);
    println!("{}", utils::bytes_to_hex(&e));
}
