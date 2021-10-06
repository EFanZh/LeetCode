pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Clone)]
struct Node {
    children: [u16; 26],
}

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut pool = Vec::with_capacity(s.len());

        let mut root = Node {
            children: [u16::MAX; 26],
        };

        for (i, c) in (0..).zip(s.bytes().rev()) {
            pool.push(root.clone());

            root.children[usize::from(c) - usize::from(b'a')] = i;
        }

        words
            .iter()
            .filter(|word| {
                let mut node = &root;

                for c in word.bytes() {
                    if let Some(next_node) = pool.get(usize::from(node.children[usize::from(c) - usize::from(b'a')])) {
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
