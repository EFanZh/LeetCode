// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

#[derive(Default)]
struct Node {
    indices: Vec<u16>,
    children: [Option<Box<Self>>; 26],
}

pub struct WordFilter {
    forward_trie: Node,
    backward_trie: Node,
}

impl WordFilter {
    fn trie_insert(mut node: &mut Node, s: impl Iterator<Item = u8>, index: u16) {
        node.indices.push(index);

        for c in s {
            node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
            node.indices.push(index);
        }
    }

    fn new(words: Vec<String>) -> Self {
        let mut result = Self {
            forward_trie: Node::default(),
            backward_trie: Node::default(),
        };

        for (i, word) in (0..).zip(words) {
            Self::trie_insert(&mut result.forward_trie, word.bytes(), i);
            Self::trie_insert(&mut result.backward_trie, word.bytes().rev(), i);
        }

        result
    }

    fn trie_search(mut node: &Node, s: impl Iterator<Item = u8>) -> Option<&[u16]> {
        for c in s {
            node = node.children[usize::from(c - b'a')].as_deref()?;
        }

        Some(&node.indices)
    }

    fn helper(&self, prefix: &str, suffix: &str) -> Option<u16> {
        let prefix_indices = Self::trie_search(&self.forward_trie, prefix.bytes())?;
        let suffix_indices = Self::trie_search(&self.backward_trie, suffix.bytes().rev())?;
        let mut iter_1 = prefix_indices.iter().copied().rev();
        let mut iter_2 = suffix_indices.iter().copied().rev();
        let mut left = iter_1.next().unwrap();
        let mut right = iter_2.next().unwrap();

        loop {
            match left.cmp(&right) {
                Ordering::Less => right = iter_2.next()?,
                Ordering::Equal => return Some(left),
                Ordering::Greater => left = iter_1.next()?,
            }
        }
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
