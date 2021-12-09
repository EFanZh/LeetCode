use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        node.map_or((None, 0), |node| {
            let node_ref = node.borrow();
            let left = node_ref.left.clone();
            let right = node_ref.right.clone();

            drop(node_ref);

            let (left_result, left_depth) = Self::helper(left);
            let (right_result, right_depth) = Self::helper(right);

            match left_depth.cmp(&right_depth) {
                Ordering::Less => (right_result, right_depth + 1),
                Ordering::Equal => (Some(node), left_depth + 1),
                Ordering::Greater => (left_result, left_depth + 1),
            }
        })
    }

    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(root).0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::subtree_with_all_deepest(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
