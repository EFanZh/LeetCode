use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    #[allow(clippy::option_if_let_else)] // False positive.
    fn helper(node: &mut Option<Rc<RefCell<TreeNode>>>, limit: i32) -> i32 {
        if let Some(node_ref) = node.as_deref() {
            let mut node_ref = node_ref.borrow_mut();
            let child_limit = limit - node_ref.val;
            let left_sum = Self::helper(&mut node_ref.left, child_limit);
            let right_sum = Self::helper(&mut node_ref.right, child_limit);
            let mut max_sum = left_sum.max(right_sum);

            if max_sum == i32::MIN {
                max_sum = node_ref.val;
            } else {
                max_sum += node_ref.val;
            }

            drop(node_ref);

            if max_sum != i32::MIN && max_sum < limit {
                *node = None;
            }

            max_sum
        } else {
            i32::MIN
        }
    }

    pub fn sufficient_subset(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;

        Self::helper(&mut root, limit);

        root
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sufficient_subset(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::sufficient_subset(root, limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
