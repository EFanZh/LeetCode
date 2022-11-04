use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    Some(1),
                    Some(4),
                    Some(3),
                    Some(2),
                    Some(4),
                    Some(2),
                    Some(5),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(4),
                    Some(6),
                ] as &[_],
                20,
            ),
            (&[Some(4), Some(3), None, Some(1), Some(2)], 2),
            (&[Some(-4), Some(-2), Some(-5)], 0),
            (
                &[
                    Some(4),
                    Some(8),
                    None,
                    Some(6),
                    Some(1),
                    Some(9),
                    None,
                    Some(-5),
                    Some(4),
                    None,
                    None,
                    None,
                    Some(-3),
                    None,
                    Some(10),
                ],
                14,
            ),
            (&[Some(5), Some(4), Some(8), Some(3), None, Some(6), Some(3)], 7),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::max_sum_bst(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
