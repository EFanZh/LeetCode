use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn diameter_of_binary_tree_helper(root: Option<&RefCell<TreeNode>>) -> (i32, i32) {
        root.map_or((0, 0), |root| {
            let root = root.borrow();
            let (left_diameter, left_height) = Self::diameter_of_binary_tree_helper(root.left.as_deref());
            let (right_diameter, right_height) = Self::diameter_of_binary_tree_helper(root.right.as_deref());

            (
                left_diameter.max(right_diameter).max(left_height + right_height),
                left_height.max(right_height) + 1,
            )
        })
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::diameter_of_binary_tree_helper(root.as_deref()).0
    }
}

impl super::Solution for Solution {
    fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::diameter_of_binary_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
