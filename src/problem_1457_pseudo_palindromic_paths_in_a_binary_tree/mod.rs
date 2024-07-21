use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(2), Some(3), Some(1), Some(3), Some(1), None, Some(1)] as &[_], 2),
            (
                &[
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(3),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(1),
                ],
                1,
            ),
            (&[Some(9)], 1),
            (
                &[
                    Some(8),
                    Some(8),
                    None,
                    Some(7),
                    Some(7),
                    None,
                    None,
                    Some(2),
                    Some(4),
                    None,
                    Some(8),
                    None,
                    Some(7),
                    None,
                    Some(1),
                ],
                2,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::pseudo_palindromic_paths(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
