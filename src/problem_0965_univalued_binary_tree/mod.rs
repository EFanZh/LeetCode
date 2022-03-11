use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(1), Some(1), Some(1), Some(1), Some(1), None, Some(1)] as &[_],
                true,
            ),
            (&[Some(2), Some(2), Some(2), Some(5), Some(2)], false),
            (&[Some(2), Some(2), Some(2), Some(5), Some(2)], false),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::is_unival_tree(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
