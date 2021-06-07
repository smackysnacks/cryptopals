pub fn solve() -> bool {
    let lines = include_str!("8.txt").lines();
    for (mut line_number, line) in lines.enumerate() {
        line_number += 1;
        let mut chunks: Vec<_> = line.as_bytes().chunks(16).collect();
        chunks.sort();
        let n = chunks.len();
        chunks.dedup();
        if chunks.len() < n && line_number == 133 {
            return true;
        }
    }

    false
}
