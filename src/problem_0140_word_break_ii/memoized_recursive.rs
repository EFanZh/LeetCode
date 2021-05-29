pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    has_value: bool,
}

impl Solution {
    fn word_break_helper(s: &[u8], root: &TrieNode, cache: &mut HashMap<*const u8, Rc<[String]>>) -> Rc<[String]> {
        if let Some(result) = cache.get(&s.as_ptr()) {
            Rc::clone(result)
        } else {
            let mut result = Vec::new();

            if s.is_empty() {
                result.push("".to_string());
            } else {
                let mut node = root;

                for (i, c) in s.iter().copied().enumerate() {
                    if let Some(child) = node.children[usize::from(c - b'a')].as_deref() {
                        if child.has_value {
                            for tail in Self::word_break_helper(&s[i + 1..], root, cache).iter() {
                                if tail.is_empty() {
                                    result.push(s[..=i].iter().copied().map(char::from).collect());
                                } else {
                                    let mut item = String::with_capacity(i + 2 + tail.len());

                                    item.extend(s[..=i].iter().copied().map(char::from));
                                    item.push(' ');
                                    item.push_str(tail);

                                    result.push(item);
                                }
                            }
                        }

                        node = child;
                    } else {
                        break;
                    }
                }
            }

            Rc::clone(cache.entry(s.as_ptr()).or_insert_with(|| result.into()))
        }
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let s = s.into_bytes();
        let mut root = TrieNode::default();

        for word in word_dict {
            let mut node = &mut root;

            for c in word.bytes() {
                node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
            }

            node.has_value = true;
        }

        let result = Self::word_break_helper(&s, &root, &mut HashMap::new());

        result.to_vec()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
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
