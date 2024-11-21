use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = node.as_deref().unwrap().borrow();
        let node = &*node;

        match node.val {
            0 => false,
            1 => true,
            operator => {
                let lhs = Self::helper(&node.left);

                if operator == 2 {
                    lhs || Self::helper(&node.right)
                } else {
                    lhs && Self::helper(&node.right)
                }
            }
        }
    }

    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(&root)
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
