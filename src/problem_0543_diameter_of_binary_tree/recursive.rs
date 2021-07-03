use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn diameter_of_binary_tree_helper(root: &TreeNode) -> (i32, i32) {
        match (root.left.clone(), root.right.clone()) {
            (None, None) => (0, 1),
            (None, Some(child)) | (Some(child), None) => {
                let (diameter, height) = Self::diameter_of_binary_tree_helper(&child.borrow());

                (diameter.max(height), height + 1)
            }
            (Some(left), Some(right)) => {
                let (left_diameter, left_height) = Self::diameter_of_binary_tree_helper(&left.borrow());
                let (right_diameter, right_height) = Self::diameter_of_binary_tree_helper(&right.borrow());

                (
                    left_diameter.max(right_diameter).max(left_height + right_height),
                    left_height.max(right_height) + 1,
                )
            }
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map_or(0, |root| Self::diameter_of_binary_tree_helper(&root.borrow()).0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
