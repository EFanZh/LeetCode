use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    const EXTRA_TEST: ((&[Option<i32>], i32, i32), &str) = (
        (
            &[
                Some(3),
                Some(6),
                Some(5),
                None,
                None,
                Some(13),
                Some(8),
                Some(9),
                Some(19),
                Some(11),
                Some(2),
                Some(10),
                Some(17),
                None,
                None,
                None,
                Some(18),
                None,
                Some(14),
                None,
                None,
                None,
                None,
                Some(15),
                None,
                Some(4),
                Some(16),
                None,
                None,
                None,
                None,
                None,
                Some(7),
                None,
                Some(1),
                Some(12),
            ],
            10,
            13,
        ),
        "UU",
    );

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[Some(5), Some(1), Some(2), Some(3), None, Some(6), Some(4)] as &[_],
                    3,
                    6,
                ),
                "UURL",
            ),
            ((&[Some(2), Some(1)], 2, 1), "L"),
            (
                (
                    &[
                        Some(5),
                        Some(8),
                        Some(3),
                        Some(1),
                        None,
                        Some(4),
                        Some(7),
                        Some(6),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(2),
                    ],
                    5,
                    6,
                ),
                "LLL",
            ),
            (
                (
                    &[
                        Some(7),
                        Some(8),
                        Some(3),
                        Some(1),
                        None,
                        Some(4),
                        Some(5),
                        Some(6),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(2),
                    ],
                    7,
                    5,
                ),
                "RR",
            ),
            EXTRA_TEST,
        ];

        for ((root, start_value, dest_value), expected) in test_cases {
            assert_eq!(
                S::get_directions(test_utilities::make_tree(root.iter().copied()), start_value, dest_value),
                expected,
            );
        }
    }
}
