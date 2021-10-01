// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
pub struct Node {
    children: [Option<Box<Node>>; 26],
    has_value: bool,
}

impl Node {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;

        for c in word.into_bytes() {
            node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
        }

        node.has_value = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = self;

        for c in word.into_bytes() {
            if let Some(child) = node.children[usize::from(c - b'a')].as_deref() {
                node = child;
            } else {
                return false;
            }
        }

        node.has_value
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;

        for c in prefix.into_bytes() {
            if let Some(child) = node.children[usize::from(c - b'a')].as_deref() {
                node = child;
            } else {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Trie for Node {
    fn new() -> Self {
        Self::new()
    }

    fn insert(&mut self, word: String) {
        self.insert(word);
    }

    fn search(&self, word: String) -> bool {
        self.search(word)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.starts_with(prefix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Node>();
    }
}
