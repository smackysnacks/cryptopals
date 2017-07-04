fn main() {
    let lines = include_str!("../8.txt").lines();
    for line in lines {
        let mut chunks: Vec<_> = line.as_bytes().chunks(32).collect();
        chunks.sort();
        let n = chunks.len();
        chunks.dedup();
        if chunks.len() < n {
            println!("encrypted with AES-128-ECB:");
            println!("{}", line);
        }
    }
}
