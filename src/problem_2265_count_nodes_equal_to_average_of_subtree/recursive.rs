use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>, result: &mut i32) -> (u32, u32) {
        node.map_or((0, 0), |node| {
            let node = node.borrow();
            let node = &*node;
            let value = node.val as u32;
            let (left_count, left_sum) = Self::helper(node.left.as_deref(), result);
            let (right_count, right_sum) = Self::helper(node.right.as_deref(), result);
            let count = left_count + right_count + 1;
            let sum = left_sum + right_sum + value;

            *result += i32::from(value == sum / count);

            (count, sum)
        })
    }

    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        Self::helper(root.as_deref(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::average_of_subtree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
