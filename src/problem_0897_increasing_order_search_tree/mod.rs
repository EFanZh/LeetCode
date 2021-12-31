use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    Some(8),
                    Some(1),
                    None,
                    None,
                    None,
                    Some(7),
                    Some(9),
                ] as &[_],
                &[
                    Some(1),
                    None,
                    Some(2),
                    None,
                    Some(3),
                    None,
                    Some(4),
                    None,
                    Some(5),
                    None,
                    Some(6),
                    None,
                    Some(7),
                    None,
                    Some(8),
                    None,
                    Some(9),
                ] as &[_],
            ),
            (&[Some(5), Some(1), Some(7)], &[Some(1), None, Some(5), None, Some(7)]),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::increasing_bst(test_utilities::make_tree(root.iter().copied())),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
