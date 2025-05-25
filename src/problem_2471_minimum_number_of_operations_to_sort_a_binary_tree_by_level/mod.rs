use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
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
                    Some(7),
                    Some(6),
                    Some(8),
                    Some(5),
                    None,
                    None,
                    None,
                    None,
                    Some(9),
                    None,
                    Some(10),
                ] as &[_],
                3,
            ),
            (&[Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4)], 3),
            (&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)], 0),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::minimum_operations(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
