use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bst_iterator;

pub trait Solution {
    fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)] as &[_], 9),
                true,
            ),
            (
                (&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 28),
                false,
            ),
            ((&[Some(2), Some(1), Some(3)], 4), true),
            ((&[Some(2), Some(1), Some(3)], 1), false),
            ((&[Some(2), Some(1), Some(3)], 3), true),
        ];

        for ((root, k), expected) in test_cases {
            assert_eq!(
                S::find_target(test_utilities::make_tree(root.iter().copied()), k),
                expected
            );
        }
    }
}
