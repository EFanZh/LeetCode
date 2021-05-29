use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>) -> (i32, i32) {
        node.map_or((0, 0), |node| {
            let node = node.borrow();
            let (left_sum, left_tilt) = Self::helper(node.left.as_deref());
            let (right_sum, right_tilt) = Self::helper(node.right.as_deref());

            (
                left_sum + right_sum + node.val,
                left_tilt + right_tilt + (right_sum - left_sum).abs(),
            )
        })
    }

    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(root.as_deref()).1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find_tilt(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
