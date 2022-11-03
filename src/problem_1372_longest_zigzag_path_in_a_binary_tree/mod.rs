use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
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
                    None,
                    Some(1),
                    Some(1),
                    Some(1),
                    None,
                    None,
                    Some(1),
                    Some(1),
                    None,
                    Some(1),
                    None,
                    None,
                    None,
                    Some(1),
                    None,
                    Some(1),
                ] as &[_],
                3,
            ),
            (
                &[
                    Some(1),
                    Some(1),
                    Some(1),
                    None,
                    Some(1),
                    None,
                    None,
                    Some(1),
                    Some(1),
                    None,
                    Some(1),
                ],
                4,
            ),
            (&[Some(1)], 0),
            (&[Some(1), Some(1), Some(1), Some(1), None, None, Some(1)], 1),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::longest_zig_zag(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
