use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;
pub mod recursive_2;

pub trait Solution {
    fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(1), Some(2), Some(3), None, Some(5)] as &[_],
                &["1->2->5", "1->3"] as &[_],
            ),
            (
                &[Some(1), Some(2), Some(3), Some(5), Some(6)],
                &["1->2->5", "1->2->6", "1->3"],
            ),
            (&[Some(1), Some(2)], &["1->2"]),
            (&[], &[]),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::binary_tree_paths(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
