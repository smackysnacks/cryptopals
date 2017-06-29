pub fn hex_to_bytes(s: &str) -> Vec<u8> {
    assert_eq!(s.len()%2, 0);

    let mut bytes = Vec::with_capacity(s.len()/2);
    for i in 0..s.len()/2 {
        bytes.push(u8::from_str_radix(&s[i*2..i*2+2], 16).unwrap());
    }
    bytes
}

pub fn bytes_to_hex(b: &Vec<u8>) -> String {
    let mut s = String::with_capacity(b.len()*2);
    for byte in b {
        s.push_str(&format!("{:02x}", byte));
    }
    s
}
