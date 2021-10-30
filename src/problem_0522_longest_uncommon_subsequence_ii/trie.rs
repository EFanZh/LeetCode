pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::rc::Rc;

#[derive(Clone, Default)]
struct Node {
    children: [Option<Rc<Node>>; 26],
}

impl Solution {
    fn build_trie(s: &[u8]) -> Node {
        let mut root = Node::default();

        for c in s.iter().rev() {
            root.children[usize::from(c - b'a')] = Some(Rc::new(root.clone()));
        }

        root
    }

    fn is_subsequence(mut root: &Node, s: &[u8]) -> bool {
        for c in s {
            if let Some(child) = root.children[usize::from(c - b'a')].as_deref() {
                root = child;
            } else {
                return false;
            }
        }

        true
    }

    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut strs = strs;

        strs.sort_unstable_by(|lhs, rhs| ((Reverse(lhs.len()), lhs.as_str()).cmp(&(Reverse(rhs.len()), rhs.as_str()))));

        let mut tries = Vec::with_capacity(strs.len() - 1);
        let mut iter = strs.iter().map(String::as_bytes);
        let mut prev = iter.next().unwrap();
        let mut start = 0;
        let mut i = 1;

        for s in iter {
            if s != prev {
                if i - start == 1 && tries.iter().all(|trie| !Self::is_subsequence(trie, prev)) {
                    return prev.len() as _;
                }

                tries.push(Self::build_trie(prev));

                start = i;
                prev = s;
            }

            i += 1;
        }

        if i - start == 1 && tries.iter().all(|trie| !Self::is_subsequence(trie, prev)) {
            prev.len() as _
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_lu_slength(strs: Vec<String>) -> i32 {
        Self::find_lu_slength(strs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
