use cryptopals::utils::pkcs7_pad;

pub fn solve() -> bool {
    let mut s1 = b"YELLOW SUBMARINE".to_vec();
    let mut s2 = b"YELLOW SUBMARINE!".to_vec();
    let mut s3 = b"YELLOW SUBMARINE!!".to_vec();
    let mut s4 = b"YELLOW SUBMARINE!!!".to_vec();
    let mut s5 = b"YELLOW SUBMARINE!!!!".to_vec();

    pkcs7_pad(&mut s1, 20);
    pkcs7_pad(&mut s2, 20);
    pkcs7_pad(&mut s3, 20);
    pkcs7_pad(&mut s4, 20);
    pkcs7_pad(&mut s5, 20);

    return b"YELLOW SUBMARINE\x04\x04\x04\x04" == s1.as_slice()
        && b"YELLOW SUBMARINE!\x03\x03\x03" == s2.as_slice()
        && b"YELLOW SUBMARINE!!\x02\x02" == s3.as_slice()
        && b"YELLOW SUBMARINE!!!\x01" == s4.as_slice()
        && &b"YELLOW SUBMARINE!!!!\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14"[..] == s5.as_slice();
}
