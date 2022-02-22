use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;
pub mod recursive_2;

pub trait Solution {
    fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)] as &[_],
                    7,
                    15,
                ),
                32,
            ),
            (
                (
                    &[
                        Some(10),
                        Some(5),
                        Some(15),
                        Some(3),
                        Some(7),
                        Some(13),
                        Some(18),
                        Some(1),
                        None,
                        Some(6),
                    ],
                    6,
                    10,
                ),
                23,
            ),
            (
                (
                    &[
                        Some(18),
                        Some(9),
                        Some(27),
                        Some(6),
                        Some(15),
                        Some(24),
                        Some(30),
                        Some(3),
                        None,
                        Some(12),
                        None,
                        Some(21),
                    ],
                    18,
                    24,
                ),
                63,
            ),
            (
                (
                    &[
                        Some(25),
                        Some(16),
                        Some(34),
                        Some(13),
                        Some(22),
                        Some(31),
                        Some(37),
                        Some(10),
                        None,
                        Some(19),
                        None,
                        Some(28),
                    ],
                    22,
                    37,
                ),
                177,
            ),
            ((&[Some(5), Some(3), Some(7)], 1, 4), 3),
            ((&[Some(5), Some(3), Some(7)], 9, 10), 0),
            ((&[Some(5), Some(3), Some(7)], 5, 5), 5),
        ];

        for ((root, low, high), expected) in test_cases {
            assert_eq!(
                S::range_sum_bst(test_utilities::make_tree(root.iter().copied()), low, high),
                expected
            );
        }
    }
}
