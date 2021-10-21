pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

#[derive(Default)]
struct Node {
    children: [Option<Box<Self>>; 26],
    has_children: bool,
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut root = Node::default();
        let mut result = 1;

        for word in &words {
            let mut iter = word.bytes().rev();
            let mut node = &mut root;
            let mut i = 2;

            for c in &mut iter {
                let child_slot = &mut node.children[usize::from(c) - usize::from(b'a')];
                let old_has_children = mem::replace(&mut node.has_children, true);

                if let Some(child) = child_slot {
                    node = child;
                    i += 1;
                } else {
                    node = child_slot.get_or_insert_with(Box::default);
                    result += if old_has_children { i } else { 1 };

                    break;
                }
            }

            for c in iter {
                node.has_children = true;
                node = node.children[usize::from(c) - usize::from(b'a')].get_or_insert_with(Box::default);
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_length_encoding(words: Vec<String>) -> i32 {
        Self::minimum_length_encoding(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
