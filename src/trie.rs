pub struct TrieNode {
    children: Box<[Option<TrieNode>; 26]>,
}

fn ctoi(c: char) -> usize {
    (c as u8 - b'a') as usize
}

fn itoc(c: usize) -> char {
    (c as u8 + b'a') as char
}

impl<I: AsRef<str>> FromIterator<I> for TrieNode {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        iter.into_iter().fold(TrieNode::new(), |mut a, w| {
            a.add(w.as_ref().chars());
            a
        })
    }
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: Box::new([const { None }; 26]),
        }
    }
    pub fn get(&self, c: char) -> Option<&TrieNode> {
        self.children[ctoi(c)].as_ref()
    }
    pub fn add(&mut self, s: impl Iterator<Item = char>) {
        let mut current = self;
        for i in s.map(ctoi) {
            current = current.children[i].get_or_insert(TrieNode::new());
        }
    }
    pub fn children(&self) -> impl Iterator<Item = (char, &TrieNode)> {
        self.children
            .iter()
            .enumerate()
            .filter_map(|(i, n)| n.as_ref().map(|n| (itoc(i), n)))
    }
}
