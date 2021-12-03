use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterator;

pub trait Solution {
    fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[
                        Some(3),
                        Some(5),
                        Some(1),
                        Some(6),
                        Some(2),
                        Some(9),
                        Some(8),
                        None,
                        None,
                        Some(7),
                        Some(4),
                    ] as &[_],
                    &[
                        Some(3),
                        Some(5),
                        Some(1),
                        Some(6),
                        Some(7),
                        Some(4),
                        Some(2),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(9),
                        Some(8),
                    ] as &[_],
                ),
                true,
            ),
            ((&[Some(1)], &[Some(1)]), true),
            ((&[Some(1)], &[Some(2)]), false),
            ((&[Some(1), Some(2)], &[Some(2), Some(2)]), true),
            ((&[Some(1), Some(2), Some(3)], &[Some(1), Some(3), Some(2)]), false),
            (
                (
                    &[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7)],
                    &[Some(4), Some(2), Some(6), None, Some(3), Some(5), Some(7)],
                ),
                false,
            ),
        ];

        for ((root1, root2), expected) in test_cases {
            assert_eq!(
                S::leaf_similar(
                    test_utilities::make_tree(root1.iter().copied()),
                    test_utilities::make_tree(root2.iter().copied())
                ),
                expected
            );
        }
    }
}
