use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(1), Some(0), Some(1), Some(0), Some(1), Some(0), Some(1)] as &[_],
                22,
            ),
            (&[Some(0)], 0),
            (&[Some(1), Some(1)], 3),
            (&[Some(1), None, Some(0)], 2),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::sum_root_to_leaf(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
