use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn is_flipped(left: Option<&RefCell<TreeNode>>, right: Option<&RefCell<TreeNode>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();

                left.val == right.val
                    && Self::is_flipped(left.left.as_deref(), right.right.as_deref())
                    && Self::is_flipped(left.right.as_deref(), right.left.as_deref())
            }
            _ => false,
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let root = root.borrow();

            Self::is_flipped(root.left.as_deref(), root.right.as_deref())
        } else {
            true
        }
    }
}

impl super::Solution for Solution {
    fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_symmetric(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
