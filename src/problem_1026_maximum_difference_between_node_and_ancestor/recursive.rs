use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: &RefCell<TreeNode>, result: &mut i32) -> (i32, i32) {
        let node = node.borrow();
        let mut min = node.val;
        let mut max = node.val;

        if let Some(left) = node.left.as_deref() {
            let (left_min, left_max) = Self::helper(left, result);

            min = min.min(left_min);
            max = max.max(left_max);
        }

        if let Some(right) = node.right.as_deref() {
            let (right_min, right_max) = Self::helper(right, result);

            min = min.min(right_min);
            max = max.max(right_max);
        }

        *result = (*result).max(node.val - min);
        *result = (*result).max(max - node.val);

        (min, max)
    }

    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        Self::helper(&root.unwrap(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_ancestor_diff(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
