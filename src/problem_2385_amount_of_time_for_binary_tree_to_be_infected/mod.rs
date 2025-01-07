use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32;
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
                        Some(5),
                        Some(3),
                        None,
                        Some(4),
                        Some(10),
                        Some(6),
                        Some(9),
                        Some(2),
                    ] as &[_],
                    3,
                ),
                4,
            ),
            ((&[Some(1)], 1), 0),
            ((&[Some(3), Some(5), Some(2), None, Some(1), Some(4)], 5), 3),
            ((&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)], 4), 3),
        ];

        for ((root, start), expected) in test_cases {
            assert_eq!(
                S::amount_of_time(test_utilities::make_tree(root.iter().copied()), start),
                expected,
            );
        }
    }
}
