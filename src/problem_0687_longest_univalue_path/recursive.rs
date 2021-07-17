use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node_and_parent: Option<(&RefCell<TreeNode>, i32)>) -> (i32, i32) {
        node_and_parent.map_or((0, 0), |(node, parent)| {
            let node = node.borrow();
            let value = node.val;
            let (left_max_path, left_depth) = Self::helper(node.left.as_deref().map(|left| (left, value)));
            let (right_max_path, right_depth) = Self::helper(node.right.as_deref().map(|right| (right, value)));

            (
                left_max_path.max(right_max_path).max(left_depth + right_depth),
                if value == parent {
                    left_depth.max(right_depth) + 1
                } else {
                    0
                },
            )
        })
    }

    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map_or(0, |node| {
            let node = node.borrow();
            let value = node.val;
            let (left_max_path, left_depth) = Self::helper(node.left.as_deref().map(|left| (left, value)));
            let (right_max_path, right_depth) = Self::helper(node.right.as_deref().map(|right| (right, value)));

            left_max_path.max(right_max_path).max(left_depth + right_depth)
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::longest_univalue_path(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
