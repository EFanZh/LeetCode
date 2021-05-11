use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    Some(4),
                    Some(1),
                    Some(6),
                    Some(0),
                    Some(2),
                    Some(5),
                    Some(7),
                    None,
                    None,
                    None,
                    Some(3),
                    None,
                    None,
                    None,
                    Some(8),
                ] as &[_],
                &[
                    Some(30),
                    Some(36),
                    Some(21),
                    Some(36),
                    Some(35),
                    Some(26),
                    Some(15),
                    None,
                    None,
                    None,
                    Some(33),
                    None,
                    None,
                    None,
                    Some(8),
                ] as &[_],
            ),
            (&[Some(0), None, Some(1)], &[Some(1), None, Some(1)]),
            (&[Some(1), Some(0), Some(2)], &[Some(3), Some(3), Some(2)]),
            (
                &[Some(3), Some(2), Some(4), Some(1)],
                &[Some(7), Some(9), Some(4), Some(10)],
            ),
        ];

        for (root, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::convert_bst(test_utilities::make_tree(root.iter().copied())),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
