use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[Some(3), Some(0), Some(4), None, Some(2), None, None, Some(1)] as &[_],
                    1,
                    3,
                ),
                &[Some(3), Some(2), None, Some(1)] as &[_],
            ),
            ((&[Some(1)], 1, 2), &[Some(1)]),
            ((&[Some(1), None, Some(2)], 1, 3), &[Some(1), None, Some(2)]),
            ((&[Some(1), None, Some(2)], 2, 4), &[Some(2)]),
            ((&[Some(3)], 2, 2), &[None]),
        ];

        for ((root, low, high), expected) in test_cases {
            assert_eq!(
                S::trim_bst(test_utilities::make_tree(root.iter().copied()), low, high),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
