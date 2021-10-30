pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Node {
    has_value: bool,
    children: [Option<Box<Node>>; 26],
}

impl Solution {
    fn trie_contains_word(mut root: &Node, word: &[u8]) -> bool {
        for c in word {
            if let Some(child) = root.children[usize::from(c - b'a')].as_deref() {
                root = child;
            } else {
                return false;
            }
        }

        root.has_value
    }

    fn is_concatenated_word(root: &Node, word: &[u8], cache: &mut Vec<bool>) -> bool {
        if word.len() < 2 {
            false
        } else {
            cache.reserve(word.len() + 1);
            cache.push(true);

            for end in 1..=word.len() {
                cache.push(
                    cache[..end]
                        .iter()
                        .enumerate()
                        .any(|(start, &value)| value && Self::trie_contains_word(root, &word[start..end])),
                );
            }

            let result = *cache.last().unwrap();

            cache.clear();

            result
        }
    }

    fn trie_insert(mut root: &mut Node, word: &[u8]) {
        for c in word {
            root = root.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
        }

        root.has_value = true;
    }

    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut words = words;

        words.sort_by_key(String::len);

        let mut trie = Node::default();
        let mut cache = Vec::new();

        words.retain(|word| {
            let result = Self::is_concatenated_word(&trie, word.as_bytes(), &mut cache);

            Self::trie_insert(&mut trie, word.as_bytes());

            result
        });

        words
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        Self::find_all_concatenated_words_in_a_dict(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
