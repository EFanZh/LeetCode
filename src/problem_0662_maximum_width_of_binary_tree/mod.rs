use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)] as &[_], 4),
            (&[Some(1), Some(3), None, Some(5), Some(3)], 2),
            (&[Some(1), Some(3), Some(2), Some(5)], 2),
            (
                &[
                    Some(1),
                    Some(3),
                    Some(2),
                    Some(5),
                    None,
                    None,
                    Some(9),
                    Some(6),
                    None,
                    None,
                    Some(7),
                ],
                8,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::width_of_binary_tree(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
