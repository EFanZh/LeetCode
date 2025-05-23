use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn is_valid_bst_helper(root: Option<&RefCell<TreeNode>>, min: i64, max: i64) -> bool {
        root.is_none_or(|node| {
            let node = node.borrow();
            let val = i64::from(node.val);

            val >= min
                && val < max
                && Self::is_valid_bst_helper(node.left.as_deref(), min, val)
                && Self::is_valid_bst_helper(node.right.as_deref(), val + 1, max)
        })
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_helper(root.as_deref(), i64::MIN, i64::MAX)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
