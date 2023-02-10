use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>, mut max: i32, good_count: &mut i32) {
        if let Some(node) = node.map(RefCell::borrow) {
            if max <= node.val {
                *good_count += 1;
                max = node.val;
            }

            Self::helper(node.left.as_deref(), max, good_count);
            Self::helper(node.right.as_deref(), max, good_count);
        }
    }

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        Self::helper(root.as_deref(), i32::MIN, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::good_nodes(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
