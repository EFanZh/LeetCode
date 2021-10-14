use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node_cell) = node.as_deref() {
            let mut node_ref = node_cell.borrow_mut();

            Self::helper(&mut node_ref.left);
            Self::helper(&mut node_ref.right);

            if node_ref.val == 0 && node_ref.left.is_none() && node_ref.right.is_none() {
                drop(node_ref);

                *node = None;
            }
        }
    }

    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;

        Self::helper(&mut root);

        root
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::prune_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
