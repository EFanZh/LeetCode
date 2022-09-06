use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>, result: &mut i32, state: u8) {
        if let Some(node) = node.map(RefCell::borrow) {
            if state & 2 == 0 {
                *result += node.val;
            }

            let left = node.left.as_deref();
            let right = node.right.as_deref();
            let child_state = (state << 1) | (node.val as u8 & 1);

            Self::helper(left, result, child_state);
            Self::helper(right, result, child_state);
        }
    }

    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        Self::helper(root.as_deref(), &mut result, u8::MAX);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_even_grandparent(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
