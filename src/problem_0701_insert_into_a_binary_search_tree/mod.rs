use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;

pub trait Solution {
    fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[Some(4), Some(2), Some(7), Some(1), Some(3)] as &[_], 5),
                &[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)] as &[_],
            ),
            (
                (
                    &[Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70)],
                    25,
                ),
                &[
                    Some(40),
                    Some(20),
                    Some(60),
                    Some(10),
                    Some(30),
                    Some(50),
                    Some(70),
                    None,
                    None,
                    Some(25),
                ],
            ),
            (
                (
                    &[
                        Some(4),
                        Some(2),
                        Some(7),
                        Some(1),
                        Some(3),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    ],
                    5,
                ),
                &[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)],
            ),
            (
                (
                    &[
                        Some(61),
                        Some(46),
                        Some(66),
                        Some(43),
                        None,
                        None,
                        None,
                        Some(39),
                        None,
                        None,
                        None,
                    ],
                    88,
                ),
                &[Some(61), Some(46), Some(66), Some(43), None, None, Some(88), Some(39)],
            ),
            ((&[], 7), &[Some(7)]),
        ];

        for ((root, val), expected) in test_cases {
            assert_eq!(
                S::insert_into_bst(test_utilities::make_tree(root.iter().copied()), val),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
