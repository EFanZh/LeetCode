use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

impl Solution {
    fn invert_tree_helper(root: Option<&RefCell<TreeNode>>) {
        if let Some(node) = root {
            let node_ref = &mut *node.borrow_mut();

            Self::invert_tree_helper(node_ref.left.as_deref());
            Self::invert_tree_helper(node_ref.right.as_deref());

            mem::swap(&mut node_ref.left, &mut node_ref.right);
        }
    }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert_tree_helper(root.as_deref());

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
