use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;
pub mod partial_iterative;
pub mod recursive;

pub trait Solution {
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[Some(1), None, Some(2), Some(3)] as &[_], &[1, 2, 3] as &[_])];

        for (root, expected) in test_cases {
            assert_eq!(
                S::preorder_traversal(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
