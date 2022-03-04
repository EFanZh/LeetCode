use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::{Ref, RefCell};
use std::rc::Rc;

impl Solution {
    fn sort_children(node: &TreeNode) -> (Option<Ref<TreeNode>>, Option<Ref<TreeNode>>) {
        let left = node.left.as_deref().map(RefCell::borrow);
        let right = node.right.as_deref().map(RefCell::borrow);

        if right.as_deref().map(|right| right.val) < left.as_deref().map(|left| left.val) {
            (right, left)
        } else {
            (left, right)
        }
    }

    fn is_equivalent(lhs: Option<&TreeNode>, rhs: Option<&TreeNode>) -> bool {
        match (lhs, rhs) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(lhs), Some(rhs)) => {
                lhs.val == rhs.val && {
                    let (lhs_left, lhs_right) = Self::sort_children(lhs);
                    let (rhs_left, rhs_right) = Self::sort_children(rhs);

                    Self::is_equivalent(lhs_left.as_deref(), rhs_left.as_deref())
                        && Self::is_equivalent(lhs_right.as_deref(), rhs_right.as_deref())
                }
            }
        }
    }

    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_equivalent(
            root1.as_deref().map(RefCell::borrow).as_deref(),
            root2.as_deref().map(RefCell::borrow).as_deref(),
        )
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::flip_equiv(root1, root2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
