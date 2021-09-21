use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), Some(2), Some(3)] as &[_], 25),
            (&[Some(4), Some(9), Some(0), Some(5), Some(1)], 1026),
            (&[Some(1), None, Some(5)], 15),
            (&[Some(0), Some(1)], 1),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::sum_numbers(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
