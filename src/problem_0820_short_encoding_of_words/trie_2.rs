pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

#[derive(Default)]
struct Node {
    children: [u16; 26],
    has_children: bool,
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut pool = Vec::with_capacity(words.iter().map(String::len).sum::<usize>() + 1);
        let mut result = 1;

        pool.push(Node::default());

        for word in &words {
            let mut iter = word.bytes().rev();
            let mut node = 0;
            let mut i = 2;

            for c in &mut iter {
                let handle = pool.len();
                let node_ref = &mut pool[node];
                let child_slot = &mut node_ref.children[usize::from(c) - usize::from(b'a')];
                let old_has_children = mem::replace(&mut node_ref.has_children, true);

                if *child_slot == 0 {
                    *child_slot = handle as _;
                    pool.push(Node::default());
                    node = handle;
                    result += if old_has_children { i } else { 1 };

                    break;
                }

                node = *child_slot as _;
                i += 1;
            }

            for c in iter {
                let handle = pool.len();
                let node_ref = &mut pool[node];

                node_ref.children[usize::from(c) - usize::from(b'a')] = handle as _;
                node_ref.has_children = true;
                pool.push(Node::default());
                node = handle;

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
