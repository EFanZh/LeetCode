use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>, min_value: i32, result: &mut Option<i32>) {
        if let Some(node) = node {
            let node = node.borrow();

            if node.val == min_value {
                Self::helper(node.left.as_deref(), min_value, result);
                Self::helper(node.right.as_deref(), min_value, result);
            } else if let Some(slot) = result {
                *slot = (*slot).min(node.val);
            } else {
                *result = Some(node.val);
            }
        }
    }

    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.unwrap();
        let root = root.borrow();
        let mut result = None;

        Self::helper(root.left.as_deref(), root.val, &mut result);
        Self::helper(root.right.as_deref(), root.val, &mut result);

        result.unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find_second_minimum_value(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
