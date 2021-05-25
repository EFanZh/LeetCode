use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn path_sum_helper(node: &TreeNode, sum: i32, base: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => {
                if node.val == sum {
                    base.push(node.val);

                    result.push(base.clone());

                    base.pop();
                }
            }
            (None, Some(child)) | (Some(child), None) => {
                base.push(node.val);

                Self::path_sum_helper(&child.borrow(), sum - node.val, base, result);

                base.pop();
            }
            (Some(left), Some(right)) => {
                base.push(node.val);

                Self::path_sum_helper(&left.borrow(), sum - node.val, base, result);
                Self::path_sum_helper(&right.borrow(), sum - node.val, base, result);

                base.pop();
            }
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if let Some(node) = root {
            Self::path_sum_helper(&node.borrow(), sum, &mut Vec::new(), &mut result);
        }

        result
    }
}

impl super::Solution for Solution {
    fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        Self::path_sum(root, sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
