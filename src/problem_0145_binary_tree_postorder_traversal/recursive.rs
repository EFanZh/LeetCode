use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn postorder_traversal_helper(root: Option<&RefCell<TreeNode>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let node_ref = node.borrow();

            Self::postorder_traversal_helper(node_ref.left.as_deref(), result);
            Self::postorder_traversal_helper(node_ref.right.as_deref(), result);

            result.push(node_ref.val);
        }
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        Self::postorder_traversal_helper(root.as_deref(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::postorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
