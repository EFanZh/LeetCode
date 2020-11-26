use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn rob_helper(root: Option<&RefCell<TreeNode>>) -> (i32, i32) {
        root.map(RefCell::borrow).map_or((0, 0), |root| {
            let (rob_root_1, rob_children_1) = Self::rob_helper(root.left.as_deref());
            let (rob_root_2, rob_children_2) = Self::rob_helper(root.right.as_deref());

            (
                root.val + rob_children_1 + rob_children_2,
                rob_root_1.max(rob_children_1) + rob_root_2.max(rob_children_2),
            )
        })
    }

    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (rob_root, rob_children) = Self::rob_helper(root.as_deref());

        rob_root.max(rob_children)
    }
}

impl super::Solution for Solution {
    fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::rob(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
