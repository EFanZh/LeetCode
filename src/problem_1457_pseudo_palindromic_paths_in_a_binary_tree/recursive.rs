use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: &RefCell<TreeNode>, mut state: u16, result: &mut i32) {
        let node = node.borrow();
        state ^= 1 << node.val;

        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => *result += i32::from(state.count_ones() < 2),
            (None, Some(child)) | (Some(child), None) => Self::helper(child, state, result),
            (Some(left), Some(right)) => {
                Self::helper(left, state, result);
                Self::helper(right, state, result);
            }
        }
    }

    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        Self::helper(root.as_deref().unwrap(), 0, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::pseudo_palindromic_paths(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
