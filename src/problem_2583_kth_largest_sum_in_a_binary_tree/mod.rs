use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64;
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
                        Some(5),
                        Some(8),
                        Some(9),
                        Some(2),
                        Some(1),
                        Some(3),
                        Some(7),
                        Some(4),
                        Some(6),
                    ] as &[_],
                    2,
                ),
                13,
            ),
            ((&[Some(1), Some(2), None, Some(3)], 1), 3),
            (
                (
                    &[Some(5), Some(8), Some(9), Some(2), Some(1), Some(3), Some(7)] as &[_],
                    4,
                ),
                -1,
            ),
        ];

        for ((root, k), expected) in test_cases {
            assert_eq!(
                S::kth_largest_level_sum(test_utilities::make_tree(root.iter().copied()), k),
                expected,
            );
        }
    }
}
