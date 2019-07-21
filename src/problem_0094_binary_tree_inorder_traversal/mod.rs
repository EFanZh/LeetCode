use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;
pub mod iterative_2;
pub mod partial_iterative;
pub mod recursive;

pub trait Solution {
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities::make_tree;
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = vec![(vec![Some(1), None, Some(2), Some(3)], vec![1, 3, 2])];

        for (serialized_tree, expected) in test_cases {
            assert_eq!(S::inorder_traversal(make_tree(serialized_tree)), expected);
        }
    }
}
