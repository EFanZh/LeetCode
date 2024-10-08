use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut root = root;
        let mut result = Vec::new();
        let mut stack = Vec::new();

        loop {
            #[expect(clippy::assigning_clones, reason = "false positive")]
            if let Some(node) = root {
                root = node.borrow().left.clone();
                stack.push(node);
            } else if let Some(node) = stack.pop() {
                let node_ref = node.borrow();

                result.push(node_ref.val);

                root = node_ref.right.clone();
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::inorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
