use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>) -> bool {
        let node = node.unwrap().borrow();
        let node = &*node;

        match node.val {
            0 => false,
            1 => true,
            operator => {
                let lhs = Self::helper(node.left.as_deref());

                if operator == 2 {
                    lhs || Self::helper(node.right.as_deref())
                } else {
                    lhs && Self::helper(node.right.as_deref())
                }
            }
        }
    }

    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(root.as_deref())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::evaluate_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
