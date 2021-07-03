use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] as &[_],
                true,
            ),
            (
                &[
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    None,
                    None,
                    Some(4),
                    Some(4),
                ],
                false,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::is_balanced(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
