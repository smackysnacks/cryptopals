fn pkcs7_pad(buffer: &mut Vec<u8>, blocksize: usize) {
    let padding = blocksize % buffer.len();
    for _ in 0..padding {
        buffer.push(padding as u8);
    }
}

fn main() {
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

    assert_eq!(b"YELLOW SUBMARINE\x04\x04\x04\x04", s1.as_slice());
    assert_eq!(b"YELLOW SUBMARINE!\x03\x03\x03", s2.as_slice());
    assert_eq!(b"YELLOW SUBMARINE!!\x02\x02", s3.as_slice());
    assert_eq!(b"YELLOW SUBMARINE!!!\x01", s4.as_slice());
    assert_eq!(b"YELLOW SUBMARINE!!!!", s5.as_slice());
}
