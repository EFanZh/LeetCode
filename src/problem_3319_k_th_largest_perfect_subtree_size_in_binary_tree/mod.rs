use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32;
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
                        Some(3),
                        Some(6),
                        Some(5),
                        Some(2),
                        Some(5),
                        Some(7),
                        Some(1),
                        Some(8),
                        None,
                        None,
                        Some(6),
                        Some(8),
                    ] as &[_],
                    2,
                ),
                3,
            ),
            ((&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)], 1), 7),
            ((&[Some(1), Some(2), Some(3), None, Some(4)], 3), -1),
        ];

        for ((root, k), expected) in test_cases {
            assert_eq!(
                S::kth_largest_perfect_subtree(test_utilities::make_tree(root.iter().copied()), k),
                expected,
            );
        }
    }
}
