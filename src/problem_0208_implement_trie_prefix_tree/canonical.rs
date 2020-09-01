#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    has_value: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;

        for c in word.into_bytes() {
            node = node.children[(c - b'a') as usize].get_or_insert_with(|| Box::new(Self::new()));
        }

        node.has_value = true
    }

    fn search(&self, word: String) -> bool {
        let mut node = self;

        for c in word.into_bytes() {
            if let Some(child) = node.children[(c - b'a') as usize].as_deref() {
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
            if let Some(child) = node.children[(c - b'a') as usize].as_deref() {
                node = child;
            } else {
                return false;
            }
        }

        true
    }
}

impl super::Trie for Trie {
    fn new() -> Self {
        Self::new()
    }

    fn insert(&mut self, word: String) {
        self.insert(word)
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
        super::super::tests::run::<super::Trie>();
    }
}
