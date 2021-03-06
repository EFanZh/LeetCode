use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn max_path_sum_helper(root: Option<&RefCell<TreeNode>>, result: &mut i32) -> i32 {
        root.map(RefCell::borrow).as_deref().map_or(i32::MIN, |root| {
            let line_sum_1 = Self::max_path_sum_helper(root.left.as_deref(), result);
            let line_sum_2 = Self::max_path_sum_helper(root.right.as_deref(), result);

            *result = (*result).max(root.val + line_sum_1.max(0) + line_sum_2.max(0));

            line_sum_1.max(line_sum_2).max(0) + root.val
        })
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MIN;

        Self::max_path_sum_helper(root.as_deref(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_path_sum(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
