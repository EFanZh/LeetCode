use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)] as &[_], 2),
                &[Some(1), None, Some(3), None, Some(4)] as &[_],
            ),
            (
                (&[Some(1), Some(3), Some(3), Some(3), Some(2)], 3),
                &[Some(1), Some(3), None, None, Some(2)],
            ),
            ((&[Some(1), Some(2), None, Some(2), None, Some(2)], 2), &[Some(1)]),
        ];

        for ((root, target), expected) in test_cases {
            assert_eq!(
                S::remove_leaf_nodes(test_utilities::make_tree(root.iter().copied()), target),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
