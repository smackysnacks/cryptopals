fn xor(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    assert_eq!(b1.len(), b2.len());

    let mut buffer = Vec::with_capacity(b1.len());
    for i in 0..b1.len() {
        buffer.push(b1[i] ^ b2[i]);
    }
    buffer
}

pub fn solve() -> bool {
    let s1 = "1c0111001f010100061a024b53535009181c";
    let s2 = "686974207468652062756c6c277320657965";

    let b1 = cryptopals::utils::hex_to_bytes(s1);
    let b2 = cryptopals::utils::hex_to_bytes(s2);
    let b3 = xor(&b1, &b2);
    cryptopals::utils::bytes_to_hex(b3) == "746865206b696420646f6e277420706c6179"
}
