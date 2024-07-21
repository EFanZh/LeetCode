use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    Some(8),
                    Some(3),
                    Some(10),
                    Some(1),
                    Some(6),
                    None,
                    Some(14),
                    None,
                    None,
                    Some(4),
                    Some(7),
                    Some(13),
                ] as &[_],
                7,
            ),
            (&[Some(1), None, Some(2), None, Some(0), Some(3)], 3),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::max_ancestor_diff(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
