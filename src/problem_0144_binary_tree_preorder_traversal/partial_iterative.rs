use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(mut root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            while let Some(node) = root {
                let node_ref = node.borrow();

                result.push(node_ref.val);
                helper(node_ref.left.clone(), result);

                root = node_ref.right.clone();
            }
        }

        let mut result = Vec::new();

        helper(root, &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::preorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
