use std::collections::HashSet;
use std::hash::Hash;

trait DuplicateCheckedIterator: Iterator {
    fn first_duplicate(&mut self) -> Option<Self::Item>;
}

impl<I: Iterator> DuplicateCheckedIterator for I
where
    I::Item: Eq + Hash + Copy,
{
    fn first_duplicate(&mut self) -> Option<Self::Item> {
        let mut hash = HashSet::new();
        self.find(|e| !hash.insert(*e))
    }
}

pub fn check(candidate: &str) -> bool {
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .first_duplicate()
        .is_none()
}
