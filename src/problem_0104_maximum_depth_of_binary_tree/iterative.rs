use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let mut depth = 0;
        let mut stack = Vec::new();

        loop {
            if let Some(node) = root {
                let node = node.borrow();

                result = result.max(stack.len());

                depth += 1;

                stack.push((node.right.clone(), depth));

                root = node.left.clone();
            } else {
                result = result.max(depth);

                if let Some((right, right_depth)) = stack.pop() {
                    root = right;
                    depth = right_depth;
                } else {
                    break;
                }
            }
        }

        result as _
    }
}

impl super::Solution for Solution {
    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_depth(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
