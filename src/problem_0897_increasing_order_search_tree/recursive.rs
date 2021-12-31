use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(
        mut node: Option<Rc<RefCell<TreeNode>>>,
        mut tail: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(node_rc) = node.as_deref() {
            let mut node_ref = node_rc.borrow_mut();
            let left = node_ref.left.take();
            let right = node_ref.right.take();
            let new_right = Self::helper(right, tail);

            node_ref.right = new_right;

            drop(node_ref);

            tail = node;
            node = left;
        }

        tail
    }

    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(root, None)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::increasing_bst(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
