use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn is_valid_bst_helper(root: Option<&RefCell<TreeNode>>, prev: Option<i32>) -> Result<Option<i32>, ()> {
        root.map_or_else(
            || Ok(prev),
            |node| {
                let node = node.borrow();

                Self::is_valid_bst_helper(node.left.as_deref(), prev)?.map_or_else(
                    || Self::is_valid_bst_helper(node.right.as_deref(), Some(node.val)),
                    |left_value| {
                        if node.val > left_value {
                            Self::is_valid_bst_helper(node.right.as_deref(), Some(node.val))
                        } else {
                            Err(())
                        }
                    },
                )
            },
        )
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_helper(root.as_deref(), None).is_ok()
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
