use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>, sum: &mut i32) {
        if let Some(mut node) = node.map(RefCell::borrow_mut) {
            Self::helper(node.right.as_deref(), sum);

            *sum += node.val;
            node.val = *sum;

            Self::helper(node.left.as_deref(), sum);
        }
    }

    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(root.as_deref(), &mut 0);

        root
    }
}

impl super::Solution for Solution {
    fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::convert_bst(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
