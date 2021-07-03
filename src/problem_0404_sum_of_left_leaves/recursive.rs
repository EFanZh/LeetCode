use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper_left(node: &TreeNode) -> i32 {
        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => node.val,
            (None, Some(right)) => Self::helper_right(&right.borrow()),
            (Some(left), None) => Self::helper_left(&left.borrow()),
            (Some(left), Some(right)) => Self::helper_left(&left.borrow()) + Self::helper_right(&right.borrow()),
        }
    }

    fn helper_right(node: &TreeNode) -> i32 {
        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => 0,
            (None, Some(right)) => Self::helper_right(&right.borrow()),
            (Some(left), None) => Self::helper_left(&left.borrow()),
            (Some(left), Some(right)) => Self::helper_left(&left.borrow()) + Self::helper_right(&right.borrow()),
        }
    }

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map_or(0, |node| Self::helper_right(&node.borrow()))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_of_left_leaves(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
