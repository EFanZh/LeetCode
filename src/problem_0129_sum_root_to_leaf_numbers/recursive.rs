use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn sum_numbers_helper(root: &TreeNode, base: i32) -> i32 {
        let base = base * 10 + root.val;

        match (
            root.left.as_deref().map(RefCell::borrow),
            root.right.as_deref().map(RefCell::borrow),
        ) {
            (None, None) => base,
            (None, Some(child)) | (Some(child), None) => Self::sum_numbers_helper(&child, base),
            (Some(left), Some(right)) => Self::sum_numbers_helper(&left, base) + Self::sum_numbers_helper(&right, base),
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map_or(0, |root| Self::sum_numbers_helper(&root.borrow(), 0))
    }
}

impl super::Solution for Solution {
    fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_numbers(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
