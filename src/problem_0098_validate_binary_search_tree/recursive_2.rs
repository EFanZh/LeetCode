use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn is_valid_bst_helper(root: Option<&RefCell<TreeNode>>, prev: Option<i32>) -> Result<Option<i32>, ()> {
        if let Some(node) = root {
            let node = node.borrow();

            if let Some(left_value) = Self::is_valid_bst_helper(node.left.as_deref(), prev)? {
                if node.val > left_value {
                    Self::is_valid_bst_helper(node.right.as_deref(), Some(node.val))
                } else {
                    Err(())
                }
            } else {
                Self::is_valid_bst_helper(node.right.as_deref(), Some(node.val))
            }
        } else {
            Ok(prev)
        }
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_helper(root.as_deref(), None).is_ok()
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
