use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod lift_right_min;

pub trait Solution {
    fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)] as &[_], 3),
                &[Some(5), Some(4), Some(6), Some(2), None, None, Some(7)] as &[_],
            ),
            (
                (&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 0),
                &[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)],
            ),
            ((&[], 0), &[]),
            (
                (&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 2),
                &[Some(5), Some(3), Some(6), None, Some(4), None, Some(7)],
            ),
            (
                (&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 6),
                &[Some(5), Some(3), Some(7), Some(2), Some(4)],
            ),
            (
                (&[Some(5), Some(3), Some(6), Some(2), Some(4), Some(7), Some(8)], 5),
                &[Some(7), Some(3), Some(6), Some(2), Some(4), None, Some(8)],
            ),
            (
                (
                    &[
                        Some(2),
                        Some(1),
                        Some(6),
                        None,
                        None,
                        Some(5),
                        None,
                        Some(3),
                        None,
                        None,
                        Some(4),
                    ],
                    2,
                ),
                &[Some(3), Some(1), Some(6), None, None, Some(5), None, Some(4)],
            ),
        ];

        for ((root, key), expected) in test_cases {
            assert_eq!(
                S::delete_node(test_utilities::make_tree(root.iter().copied()), key),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
