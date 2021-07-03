use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn is_balanced_helper(root: Option<&RefCell<TreeNode>>) -> Option<isize> {
        if let Some(node) = root {
            let node = node.borrow();
            let left_height = Self::is_balanced_helper(node.left.as_deref())?;
            let right_height = Self::is_balanced_helper(node.right.as_deref())?;

            match right_height - left_height {
                -1 | 0 => Some(left_height + 1),
                1 => Some(right_height + 1),
                _ => None,
            }
        } else {
            Some(0)
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balanced_helper(root.as_deref()).is_some()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balanced(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
