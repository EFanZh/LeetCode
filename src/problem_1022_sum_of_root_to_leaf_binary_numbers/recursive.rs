use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: &RefCell<TreeNode>, base: i32, target: &mut i32) {
        let node = node.borrow();
        let value = base * 2 + node.val;

        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => *target += value,
            (None, Some(child)) | (Some(child), None) => Self::helper(child, value, target),
            (Some(left), Some(right)) => {
                Self::helper(left, value, target);
                Self::helper(right, value, target);
            }
        }
    }

    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map_or(0, |root| {
            let mut result = 0;

            Self::helper(&root, 0, &mut result);

            result
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_root_to_leaf(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
