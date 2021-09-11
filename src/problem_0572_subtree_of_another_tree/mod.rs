use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod hash_map;
pub mod hashing;

pub trait Solution {
    fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[Some(3), Some(4), Some(5), Some(1), Some(2)] as &[_],
                    &[Some(4), Some(1), Some(2)] as &[_],
                ),
                true,
            ),
            (
                (
                    &[Some(3), Some(4), Some(5), Some(1), Some(2), None, None, None, Some(0)],
                    &[Some(4), Some(1), Some(2)],
                ),
                false,
            ),
            (
                (
                    &[
                        Some(29),
                        Some(28),
                        Some(30),
                        Some(27),
                        Some(29),
                        Some(29),
                        Some(31),
                        Some(26),
                        Some(26),
                        Some(28),
                        Some(28),
                        Some(28),
                        Some(28),
                        Some(30),
                        Some(32),
                        Some(25),
                        Some(25),
                        Some(25),
                        Some(25),
                        Some(27),
                        Some(29),
                        None,
                        Some(29),
                        Some(29),
                    ],
                    &[Some(29)],
                ),
                true,
            ),
            (
                (
                    &[
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        Some(2),
                    ],
                    &[
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        None,
                        Some(1),
                        Some(2),
                    ],
                ),
                true,
            ),
        ];

        for ((s, t), expected) in test_cases {
            assert_eq!(
                S::is_subtree(
                    test_utilities::make_tree(s.iter().copied()),
                    test_utilities::make_tree(t.iter().copied())
                ),
                expected
            );
        }
    }
}
