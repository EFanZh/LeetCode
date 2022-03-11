use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>, expected: i32) -> bool {
        node.map_or(true, |node| {
            let node = node.borrow();

            node.val == expected
                && Self::helper(node.left.as_deref(), expected)
                && Self::helper(node.right.as_deref(), expected)
        })
    }

    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.map_or(true, |root| {
            let root = root.borrow();

            Self::helper(root.left.as_deref(), root.val) && Self::helper(root.right.as_deref(), root.val)
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_unival_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
