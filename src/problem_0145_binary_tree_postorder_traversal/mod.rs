use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;
pub mod iterative_2;
pub mod recursive;

pub trait Solution {
    fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), None, Some(2), Some(3)] as &[_], &[3, 2, 1] as &[_]),
            (
                &[
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    None,
                    Some(8),
                    None,
                    None,
                    Some(6),
                    Some(7),
                    Some(9),
                ],
                &[4, 6, 7, 5, 2, 9, 8, 3, 1],
            ),
            (&[], &[]),
            (&[Some(1)], &[1]),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::postorder_traversal(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
