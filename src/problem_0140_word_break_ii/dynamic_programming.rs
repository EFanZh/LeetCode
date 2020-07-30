pub struct Solution {}

use std::str;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    has_value: bool,
}

impl Solution {
    fn word_break_helper(s: &[u8], graph: &[Vec<usize>], i: usize, base: &mut Vec<usize>, result: &mut Vec<String>) {
        if i == s.len() {
            let mut item = String::with_capacity(s.len() + base.len() - 1);

            item.push_str(str::from_utf8(&s[..base[0]]).unwrap());

            for (start, end) in base.iter().zip(&base[1..]) {
                item.push(' ');
                item.push_str(str::from_utf8(&s[*start..*end]).unwrap());
            }

            result.push(item);
        } else {
            for &next in &graph[i] {
                base.push(next);

                Self::word_break_helper(s, graph, next, base, result);

                base.pop();
            }
        }
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let s = s.into_bytes();
        let mut root = TrieNode::default();

        for word in word_dict {
            let mut node = &mut root;

            for c in word.bytes() {
                node = node.children[(c - b'a') as usize].get_or_insert_with(|| Box::new(TrieNode::default()));
            }

            node.has_value = true;
        }

        let mut graph = vec![Vec::new(); s.len()];

        for i in (0..s.len()).rev() {
            let mut trie_node = &root;

            for (j, c) in s.iter().enumerate().skip(i) {
                if let Some(child) = trie_node.children[(c - b'a') as usize].as_deref() {
                    if child.has_value && graph.get(j + 1).map_or(true, |next| !next.is_empty()) {
                        graph[i].push(j + 1);
                    }

                    trie_node = child;
                } else {
                    break;
                }
            }
        }

        let mut result = Vec::new();

        Self::word_break_helper(&s, &graph, 0, &mut Vec::new(), &mut result);

        result
    }
}

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
