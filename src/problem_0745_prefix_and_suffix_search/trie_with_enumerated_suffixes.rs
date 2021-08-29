// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Node {
    index: u16,
    children: [Option<Box<Self>>; 27],
}

pub struct WordFilter {
    trie: Node,
}

impl WordFilter {
    fn trie_insert(mut node: &mut Node, s: impl IntoIterator<Item = u8>) -> &mut Node {
        for c in s {
            node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
        }

        node
    }

    fn trie_insert_with_index(mut node: &mut Node, s: impl IntoIterator<Item = u8>, index: u16) -> &mut Node {
        node.index = node.index.max(index);

        for c in s {
            node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
            node.index = node.index.max(index);
        }

        node
    }

    fn new(words: Vec<String>) -> Self {
        let mut result = Self { trie: Node::default() };

        for (i, word) in (0..).zip(words) {
            let word = word.into_bytes();

            for suffix_length in 0..=word.len() {
                let suffix = &word[word.len() - suffix_length..];
                let node = Self::trie_insert(&mut result.trie, suffix.iter().copied());
                let node = Self::trie_insert(node, Some(b'z' + 1));
                Self::trie_insert_with_index(node, word.iter().copied(), i);
            }
        }

        result
    }

    fn trie_search(mut node: &Node, s: impl IntoIterator<Item = u8>) -> Option<&Node> {
        for c in s {
            node = node.children[usize::from(c - b'a')].as_deref()?;
        }

        Some(node)
    }

    fn helper(&self, prefix: &str, suffix: &str) -> Option<u16> {
        let node = Self::trie_search(&self.trie, suffix.bytes())?;
        let node = Self::trie_search(node, Some(b'z' + 1))?;

        Self::trie_search(node, prefix.bytes()).map(|node| node.index)
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        self.helper(&prefix, &suffix).map_or(-1, Into::into)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::WordFilter for WordFilter {
    fn new(words: Vec<String>) -> Self {
        Self::new(words)
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        self.f(prefix, suffix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::WordFilter>();
    }
}
