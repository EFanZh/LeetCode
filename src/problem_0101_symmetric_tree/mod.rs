use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)] as &[_],
                true,
            ),
            (&[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)], false),
        ];

        for (root, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::is_symmetric(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
