use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn sufficient_subset(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>>;
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
                        Some(1),
                        Some(2),
                        Some(3),
                        Some(4),
                        Some(-99),
                        Some(-99),
                        Some(7),
                        Some(8),
                        Some(9),
                        Some(-99),
                        Some(-99),
                        Some(12),
                        Some(13),
                        Some(-99),
                        Some(14),
                    ] as &[_],
                    1,
                ),
                &[
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    None,
                    None,
                    Some(7),
                    Some(8),
                    Some(9),
                    None,
                    Some(14),
                ] as &[_],
            ),
            (
                (
                    &[
                        Some(5),
                        Some(4),
                        Some(8),
                        Some(11),
                        None,
                        Some(17),
                        Some(4),
                        Some(7),
                        Some(1),
                        None,
                        None,
                        Some(5),
                        Some(3),
                    ],
                    22,
                ),
                &[
                    Some(5),
                    Some(4),
                    Some(8),
                    Some(11),
                    None,
                    Some(17),
                    Some(4),
                    Some(7),
                    None,
                    None,
                    None,
                    Some(5),
                ],
            ),
            (
                (&[Some(1), Some(2), Some(-3), Some(-5), None, Some(4), None], -1),
                &[Some(1), None, Some(-3), Some(4)],
            ),
        ];

        for ((root, limit), expected) in test_cases {
            assert_eq!(
                S::sufficient_subset(test_utilities::make_tree(root.iter().copied()), limit),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
