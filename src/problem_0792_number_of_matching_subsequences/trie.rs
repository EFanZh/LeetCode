pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::rc::Rc;

#[derive(Clone, Default)]
struct Node {
    children: [Option<Rc<Node>>; 26],
}

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut root = Node::default();

        for c in s.bytes().rev() {
            root.children[usize::from(c) - usize::from(b'a')] = Some(Rc::new(root.clone()));
        }

        words
            .iter()
            .filter(|word| {
                let mut node = &root;

                for c in word.bytes() {
                    if let Some(next_node) = node.children[usize::from(c) - usize::from(b'a')].as_deref() {
                        node = next_node;
                    } else {
                        return false;
                    }
                }

                true
            })
            .count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        Self::num_matching_subseq(s, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
