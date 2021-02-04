use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;
pub mod morris_traversal;
pub mod morris_traversal_keep_structure;
pub mod partial_iterative;
pub mod recursive;

pub trait Solution {
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), None, Some(2), Some(3)] as &[_], &[1, 3, 2] as &[_]),
            (&[Some(3), Some(1), None, None, Some(2)], &[1, 2, 3]),
            (&[], &[]),
        ];

        for (root, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::inorder_traversal(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
