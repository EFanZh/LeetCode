pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str;

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
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
        let mut root = Node::default();

        for word in word_dict {
            let mut node = &mut root;

            for c in word.bytes() {
                node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
            }

            node.has_value = true;
        }

        let mut graph = vec![Vec::new(); s.len()];

        for i in (0..s.len()).rev() {
            let mut trie_node = &root;

            for (j, c) in s.iter().enumerate().skip(i) {
                if let Some(child) = trie_node.children[usize::from(c - b'a')].as_deref() {
                    if child.has_value && graph.get(j + 1).is_none_or(|next| !next.is_empty()) {
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
