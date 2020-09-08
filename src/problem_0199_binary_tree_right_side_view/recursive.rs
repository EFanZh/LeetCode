use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn right_side_view_helper(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let (left, right, next_depth) = {
                let node = node.borrow();

                if depth == result.len() {
                    result.push(node.val);
                }

                (node.left.clone(), node.right.clone(), depth + 1)
            };

            Self::right_side_view_helper(right, next_depth, result);
            Self::right_side_view_helper(left, next_depth, result);
        }
    }

    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        Self::right_side_view_helper(root, 0, &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::right_side_view(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
