use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

impl Solution {
    fn helper(node: &RefCell<TreeNode>) {
        let node_ref = &mut *node.borrow_mut();

        if let Some(left) = node_ref.left.as_deref() {
            Self::helper(left);
        }

        if let Some(right) = node_ref.right.as_deref() {
            Self::helper(right);
        }

        mem::swap(&mut node_ref.left, &mut node_ref.right);
    }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_deref() {
            Self::helper(node);
        }

        root
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
