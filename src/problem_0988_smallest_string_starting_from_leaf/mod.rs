use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bottom_up;

pub trait Solution {
    fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(0), Some(1), Some(2), Some(3), Some(4), Some(3), Some(4)] as &[_],
                "dba",
            ),
            (&[Some(25), Some(1), Some(3), Some(1), Some(3), Some(0), Some(2)], "adz"),
            (
                &[Some(2), Some(2), Some(1), None, Some(1), Some(0), None, Some(0)],
                "abc",
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::smallest_from_leaf(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
