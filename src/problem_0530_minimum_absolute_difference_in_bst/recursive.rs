use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>, prev: &mut Option<i32>, result: &mut i32) {
        if let Some(node) = node.map(RefCell::borrow) {
            Self::helper(node.left.as_deref(), prev, result);

            if let Some(prev_val) = prev {
                *result = (*result).min(node.val - *prev_val);
            }

            *prev = Some(node.val);

            Self::helper(node.right.as_deref(), prev, result);
        }
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MAX;

        Self::helper(root.as_deref(), &mut None, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_minimum_difference(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
