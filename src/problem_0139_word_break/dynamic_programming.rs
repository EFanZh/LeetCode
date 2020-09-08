pub struct Solution;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    has_value: bool,
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.into_bytes();
        let mut root = TrieNode::default();

        for word in word_dict {
            let mut node = &mut root;

            for c in word.bytes() {
                node = node.children[(c - b'a') as usize].get_or_insert_with(|| Box::new(TrieNode::default()));
            }

            node.has_value = true;
        }

        let mut cache = vec![false; s.len() + 1];

        cache[s.len()] = true;

        for i in (0..s.len()).rev() {
            let mut node = &root;

            for (j, c) in s.iter().enumerate().skip(i) {
                if let Some(child) = node.children[(c - b'a') as usize].as_deref() {
                    if child.has_value && cache[j + 1] {
                        cache[i] = true;

                        break;
                    }

                    node = child;
                } else {
                    break;
                }
            }
        }

        cache[0]
    }
}

impl super::Solution for Solution {
    fn word_break(s: String, word_dict: Vec<String>) -> bool {
        Self::word_break(s, word_dict)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
