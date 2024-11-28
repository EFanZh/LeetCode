use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    #[expect(clippy::ref_option, reason = "by-design")]
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        if let Some(node_ref) = node.as_deref().map(RefCell::borrow) {
            let (left_node, left_height) = Self::helper(&node_ref.left);
            let (right_node, right_height) = Self::helper(&node_ref.right);

            drop(node_ref);

            match left_height.cmp(&right_height) {
                Ordering::Less => (right_node, right_height + 1),
                Ordering::Equal => (node.clone(), left_height + 1),
                Ordering::Greater => (left_node, left_height + 1),
            }
        } else {
            (None, 0)
        }
    }

    pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&root).0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::lca_deepest_leaves(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
