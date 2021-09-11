use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[Some(4), Some(2), Some(6), Some(3), Some(1), Some(5)] as &[_], 1, 2),
                &[
                    Some(4),
                    Some(1),
                    Some(1),
                    Some(2),
                    None,
                    None,
                    Some(6),
                    Some(3),
                    Some(1),
                    Some(5),
                ] as &[_],
            ),
            (
                (&[Some(4), Some(2), None, Some(3), Some(1)], 1, 3),
                &[Some(4), Some(2), None, Some(1), Some(1), Some(3), None, None, Some(1)],
            ),
            (
                (&[Some(2), None, Some(3), None, Some(5), None, Some(7)], 11, 4),
                &[
                    Some(2),
                    None,
                    Some(3),
                    None,
                    Some(5),
                    Some(11),
                    Some(11),
                    None,
                    None,
                    None,
                    Some(7),
                ],
            ),
        ];

        for ((root, val, depth), expected) in test_cases {
            let root = test_utilities::make_tree(root.iter().copied());
            let expected = test_utilities::make_tree(expected.iter().copied());
            let inverted_root = test_utilities::invert_tree(root.as_deref());
            let inverted_expected = test_utilities::invert_tree(expected.as_deref());

            for (root, expected) in [(root, expected), (inverted_root, inverted_expected)] {
                assert_eq!(S::add_one_row(root, val, depth), expected);
            }
        }
    }
}
