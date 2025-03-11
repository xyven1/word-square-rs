use std::hash::Hash;

// only enable gxhash if feature is enabled

#[cfg(feature = "gxhash")]
use gxhash::{HashMap, HashMapExt};
#[cfg(not(feature = "gxhash"))]
use std::collections::HashMap;

pub struct TrieNode<T> {
    children: HashMap<T, TrieNode<T>>,
}

impl<I: AsRef<str>> FromIterator<I> for TrieNode<char> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        iter.into_iter().fold(TrieNode::new(), |mut a, w| {
            a.add(w.as_ref().chars());
            a
        })
    }
}

impl<T: Hash + Eq> TrieNode<T> {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
        }
    }
    pub fn get(&self, c: &T) -> Option<&TrieNode<T>> {
        self.children.get(c)
    }
    pub fn add(&mut self, s: impl Iterator<Item = T>) {
        let mut current = self;
        for i in s {
            current = current.children.entry(i).or_insert(TrieNode::new());
        }
    }
    pub fn children(&self) -> impl Iterator<Item = (&T, &TrieNode<T>)> {
        self.children.iter()
    }
}
