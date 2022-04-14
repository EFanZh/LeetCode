use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(preorder: &[i32], high: i32) -> (Option<Rc<RefCell<TreeNode>>>, &[i32]) {
        if let Some((&val, rest)) = preorder.split_first() {
            if val <= high {
                let (left, rest) = Self::helper(rest, val);
                let (right, rest) = Self::helper(rest, high);

                return (Some(Rc::new(RefCell::new(TreeNode { val, left, right }))), rest);
            }
        }

        (None, preorder)
    }

    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&preorder, i32::MAX).0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::bst_from_preorder(preorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
