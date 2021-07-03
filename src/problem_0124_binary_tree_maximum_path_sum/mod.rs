use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), Some(2), Some(3)] as &[_], 6),
            (&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)], 42),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::max_path_sum(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
