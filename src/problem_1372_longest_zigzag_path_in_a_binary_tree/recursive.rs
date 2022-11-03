use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: &RefCell<TreeNode>, max_length: &mut u32) -> (u32, u32) {
        let node = node.borrow();

        let result = match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => (0, 0),
            (None, Some(right)) => (0, Self::helper(right, max_length).0 + 1),
            (Some(left), None) => (Self::helper(left, max_length).1 + 1, 0),
            (Some(left), Some(right)) => (
                Self::helper(left, max_length).1 + 1,
                Self::helper(right, max_length).0 + 1,
            ),
        };

        *max_length = (*max_length).max(result.0.max(result.1));

        result
    }

    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        Self::helper(&root.unwrap(), &mut result);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::longest_zig_zag(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
