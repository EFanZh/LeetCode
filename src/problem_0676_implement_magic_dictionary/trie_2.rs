// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

#[derive(Default)]
struct Node {
    removed: Vec<HashSet<u8>>,
    children: [Option<Box<Self>>; 26],
}

impl Node {
    fn get_or_insert<'a>(&'a mut self, word: &[u8]) -> &'a mut Self {
        let mut node = self;

        for c in word {
            node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
        }

        node
    }

    fn get(&self, word: &[u8]) -> Option<&Self> {
        let mut node = self;

        for c in word {
            if let Some(child) = node.children[usize::from(c - b'a')].as_deref() {
                node = child;
            } else {
                return None;
            }
        }

        Some(node)
    }
}

pub struct MagicDictionary {
    root: Node,
}

impl MagicDictionary {
    fn new() -> Self {
        Self { root: Node::default() }
    }

    fn vec_get_or_default<T: Default>(items: &mut Vec<T>, index: usize) -> &mut T {
        if items.len() < index + 1 {
            items.resize_with(index + 1, T::default);
        }

        &mut items[index]
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in dictionary {
            let word = word.into_bytes();

            for (i, &c) in word.iter().enumerate() {
                Self::vec_get_or_default(
                    &mut self
                        .root
                        .get_or_insert(&word[..i])
                        .get_or_insert(&word[i + 1..])
                        .removed,
                    i,
                )
                .insert(c);
            }
        }
    }

    fn search(&self, search_word: String) -> bool {
        let search_word = search_word.into_bytes();
        let root = &self.root;

        search_word.iter().enumerate().any(|(i, c)| {
            root.get(&search_word[..i])
                .and_then(|node| node.get(&search_word[i + 1..]))
                .and_then(|node| node.removed.get(i))
                .map_or(false, |removed| {
                    if removed.contains(c) {
                        removed.len() > 1
                    } else {
                        !removed.is_empty()
                    }
                })
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MagicDictionary for MagicDictionary {
    fn new() -> Self {
        Self::new()
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.build_dict(dictionary);
    }

    fn search(&self, search_word: String) -> bool {
        self.search(search_word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MagicDictionary>();
    }
}
