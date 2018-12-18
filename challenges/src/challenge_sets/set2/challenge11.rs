use rand::Rng;

fn random_key() -> [u8; 16] {
    rand::thread_rng().gen()
}

pub fn solve() -> bool {
    let key = random_key();

    false
}
