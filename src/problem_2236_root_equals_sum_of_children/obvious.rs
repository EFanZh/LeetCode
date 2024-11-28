use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn get_val(node: Option<&RefCell<TreeNode>>) -> i32 {
        node.unwrap().borrow().val
    }

    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let root = root.borrow();
        let root = &*root;

        root.val == Self::get_val(root.left.as_deref()) + Self::get_val(root.right.as_deref())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
