use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn max_depth_helper(root: Option<&RefCell<TreeNode>>) -> i32 {
        root.map_or(0, |node| {
            let node = node.borrow();

            Self::max_depth_helper(node.left.clone().as_deref())
                .max(Self::max_depth_helper(node.right.clone().as_deref()))
                + 1
        })
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_depth_helper(root.as_deref())
    }
}

impl super::Solution for Solution {
    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_depth(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
