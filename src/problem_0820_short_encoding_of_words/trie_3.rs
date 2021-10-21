pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let capacity = words.iter().map(String::len).sum::<usize>() + 1;
        let mut pool_children = vec![[0_u16; 26]; capacity];
        let mut pool_has_children = vec![false; capacity];
        let mut node_count = 1;
        let mut result = 1;

        for word in &words {
            let mut iter = word.bytes().rev();
            let mut node = 0;
            let mut i = 2;

            for c in &mut iter {
                let child_slot = &mut pool_children[node][usize::from(c) - usize::from(b'a')];
                let old_has_children = mem::replace(&mut pool_has_children[node], true);

                if *child_slot == 0 {
                    *child_slot = node_count as _;
                    node = node_count;
                    node_count += 1; // Allocate a new node.
                    result += if old_has_children { i } else { 1 };

                    break;
                }

                node = *child_slot as _;
                i += 1;
            }

            for c in iter {
                pool_children[node][usize::from(c) - usize::from(b'a')] = node_count as _;
                pool_has_children[node] = true;
                node = node_count;
                node_count += 1; // Allocate a new node.
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
