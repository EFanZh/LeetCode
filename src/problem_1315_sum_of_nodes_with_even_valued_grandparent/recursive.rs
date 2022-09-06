use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn no_op(_: &mut i32, _: i32) {}

    fn add_to_result(result: &mut i32, value: i32) {
        *result += value;
    }

    fn helper(
        node: Option<&RefCell<TreeNode>>,
        result: &mut i32,
        f1: impl FnOnce(&mut i32, i32) + Copy,
        f2: impl FnOnce(&mut i32, i32) + Copy,
    ) {
        if let Some(node) = node.map(RefCell::borrow) {
            f1(result, node.val);

            let left = node.left.as_deref();
            let right = node.right.as_deref();

            if node.val % 2 == 0 {
                Self::helper(left, result, f2, Self::add_to_result);
                Self::helper(right, result, f2, Self::add_to_result);
            } else {
                Self::helper(left, result, f2, Self::no_op);
                Self::helper(right, result, f2, Self::no_op);
            }
        }
    }

    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        Self::helper(root.as_deref(), &mut result, Self::no_op, Self::no_op);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_even_grandparent(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
