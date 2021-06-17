use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] as &[_], 24),
            (
                &[
                    Some(0),
                    Some(2),
                    Some(4),
                    Some(1),
                    None,
                    Some(3),
                    Some(-1),
                    Some(5),
                    Some(1),
                    None,
                    Some(6),
                    None,
                    Some(8),
                ],
                5,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::sum_of_left_leaves(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
