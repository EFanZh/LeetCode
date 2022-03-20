use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>, result: &mut i32) -> i32 {
        node.map_or(0, |node| {
            let node = node.borrow();
            let left_diff = Self::helper(node.left.as_deref(), result);
            let right_diff = Self::helper(node.right.as_deref(), result);
            let diff = left_diff + right_diff + node.val - 1;

            *result += diff.abs();

            diff
        })
    }

    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        Self::helper(root.as_deref(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::distribute_coins(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
