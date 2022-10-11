use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HashableHashSet<T: Hash + Eq + Clone + Ord> {
    hashset: HashSet<T>,
}

impl<T: Hash + Eq + Clone + Ord> Hash for HashableHashSet<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut sorted = vec![];

        for i in self.hashset.iter() {
            sorted.push((*i).clone());
        }

        sorted.sort();

        sorted.hash(state)
    }
}

impl<T: Hash + Eq + Clone + Ord> HashableHashSet<T> {
    pub fn new(hs: HashSet<T>) -> Self {
        HashableHashSet { hashset: hs }
    }

    pub fn hashset(&self) -> &HashSet<T> {
        &self.hashset
    }
}
