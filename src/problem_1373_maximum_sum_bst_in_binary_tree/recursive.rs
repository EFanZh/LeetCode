use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: &RefCell<TreeNode>, max_sum: &mut i32) -> Option<(i32, i32, i32)> {
        let node = node.borrow();
        let value = node.val;
        let left_result = node.left.as_deref().map(|left| Self::helper(left, max_sum));
        let right_result = node.right.as_deref().map(|right| Self::helper(right, max_sum));

        drop(node);

        let result = match (left_result, right_result) {
            (None, None) => (value, value, value),
            (None, Some(Some((right_low, right_high, right_sum)))) if value < right_low => {
                (value, right_high, value + right_sum)
            }
            (Some(Some((left_low, left_high, left_sum))), None) if left_high < value => {
                (left_low, value, left_sum + value)
            }
            (Some(Some((left_low, left_high, left_sum))), Some(Some((right_low, right_high, right_sum))))
                if left_high < value && value < right_low =>
            {
                (left_low, right_high, left_sum + value + right_sum)
            }
            _ => return None,
        };

        *max_sum = (*max_sum).max(result.2);

        Some(result)
    }

    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        Self::helper(&root.unwrap(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_sum_bst(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
