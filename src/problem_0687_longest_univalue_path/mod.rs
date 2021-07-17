use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(5), Some(4), Some(5), Some(1), Some(1), Some(5)], 2),
            (&[Some(1), Some(4), Some(5), Some(4), Some(4), Some(5)], 2),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::longest_univalue_path(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
