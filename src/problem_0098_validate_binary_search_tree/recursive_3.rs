use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn is_valid_bst_helper_0(root: &TreeNode) -> Option<i32> {
        if let Some(left) = root.left.as_deref() {
            if root.val <= Self::is_valid_bst_helper_0(&left.borrow())? {
                return None;
            }
        }

        if let Some(right) = root.right.as_deref() {
            Self::is_valid_bst_helper(&right.borrow(), root.val)
        } else {
            Some(root.val)
        }
    }

    fn is_valid_bst_helper(root: &TreeNode, mut prev: i32) -> Option<i32> {
        if let Some(left) = root.left.as_deref() {
            prev = Self::is_valid_bst_helper(&left.borrow(), prev)?;
        }

        if root.val > prev {
            if let Some(right) = root.right.as_deref() {
                Self::is_valid_bst_helper(&right.borrow(), root.val)
            } else {
                Some(root.val)
            }
        } else {
            None
        }
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            Self::is_valid_bst_helper_0(&root.borrow()).is_some()
        } else {
            true
        }
    }
}

impl super::Solution for Solution {
    fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
