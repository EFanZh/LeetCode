use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[Some(1), Some(2), Some(3), None, Some(4)] as &[_], 3), 1),
            ((&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)], 3), 2),
            (
                (
                    &[
                        Some(7),
                        Some(1),
                        Some(4),
                        Some(6),
                        None,
                        Some(5),
                        Some(3),
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(2),
                    ],
                    3,
                ),
                1,
            ),
            (
                (
                    &[
                        Some(59),
                        Some(11),
                        Some(44),
                        Some(13),
                        Some(46),
                        Some(57),
                        Some(88),
                        Some(59),
                        Some(25),
                    ],
                    1,
                ),
                0,
            ),
        ];

        for ((root, distance), expected) in test_cases {
            assert_eq!(
                S::count_pairs(test_utilities::make_tree(root.iter().copied()), distance),
                expected,
            );
        }
    }
}
