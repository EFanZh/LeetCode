use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn preorder_traversal_helper(mut root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        #[expect(clippy::assigning_clones, reason = "false positive")]
        while let Some(node) = root {
            let node_ref = node.borrow();

            result.push(node_ref.val);
            Self::preorder_traversal_helper(node_ref.left.clone(), result);

            root = node_ref.right.clone();
        }
    }

    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        Self::preorder_traversal_helper(root, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::preorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
