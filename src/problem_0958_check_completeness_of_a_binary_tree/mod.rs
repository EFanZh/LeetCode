use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)] as &[_], true),
            (&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7)], false),
            (&[Some(1)], true),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::is_complete_tree(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
