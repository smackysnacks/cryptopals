pub fn solve() -> bool {
    let solution = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63\
                    343c2a26226324272765272a282b2f20430a652e2c652a3124333\
                    a653e2b2027630c692b20283165286326302e27282f";
    let message = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let key = b"ICE";

    let e: Vec<_> = crypto::utils::xor_repeating(&message[..], &key[..]).collect();
    solution == crypto::utils::bytes_to_hex(e)
}
