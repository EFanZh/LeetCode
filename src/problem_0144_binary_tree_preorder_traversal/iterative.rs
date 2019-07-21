use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn preorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut cont = Vec::new();

        loop {
            if let Some(node) = root {
                result.push(node.borrow().val);

                root = node.borrow().left.clone();

                cont.push(node.borrow().right.clone());
            } else if let Some(right) = cont.pop() {
                root = right;
            } else {
                break;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Solution::preorder_traversal(root)
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
