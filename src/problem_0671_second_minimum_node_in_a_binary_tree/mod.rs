use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod specialized_recursive;

pub trait Solution {
    fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(2), Some(2), Some(5), None, None, Some(5), Some(7)] as &[_], 5),
            (&[Some(2), Some(2), Some(2)], -1),
            (&[Some(5), Some(8), Some(5)], 8),
            (&[Some(1), Some(1), Some(1), Some(1), Some(5), Some(1), Some(4)], 4),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::find_second_minimum_value(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
