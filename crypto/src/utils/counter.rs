use std::collections::HashMap;
use std::hash::Hash;

pub trait Counter {
    type Item;

    fn counts(&self) -> HashMap<&Self::Item, usize>;
}

impl<T> Counter for [T]
where
    T: Eq + Hash + Sized,
{
    type Item = T;

    fn counts(&self) -> HashMap<&Self::Item, usize> {
        let mut counts = HashMap::new();
        for k in self {
            *counts.entry(k).or_insert(0) += 1;
        }
        counts
    }
}
