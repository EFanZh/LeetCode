use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(1), None, Some(0), Some(0), Some(1)] as &[_],
                &[Some(1), None, Some(0), None, Some(1)] as &[_],
            ),
            (
                &[Some(1), Some(0), Some(1), Some(0), Some(0), Some(0), Some(1)],
                &[Some(1), None, Some(1), None, Some(1)],
            ),
            (
                &[Some(1), Some(1), Some(0), Some(1), Some(1), Some(0), Some(1), Some(0)],
                &[Some(1), Some(1), Some(0), Some(1), Some(1), None, Some(1)],
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::prune_tree(test_utilities::make_tree(root.iter().copied())),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
