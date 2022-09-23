use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: &mut Option<Rc<RefCell<TreeNode>>>, target: i32) {
        if let Some(node_cell) = node.as_deref() {
            let mut node_borrow = node_cell.borrow_mut();
            let node_ref = &mut *node_borrow;

            Self::helper(&mut node_ref.left, target);
            Self::helper(&mut node_ref.right, target);

            let need_to_delete = node_ref.val == target && node_ref.left.is_none() && node_ref.right.is_none();

            drop(node_borrow);

            if need_to_delete {
                *node = None;
            }
        }
    }

    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;

        Self::helper(&mut root, target);

        root
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::remove_leaf_nodes(root, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
