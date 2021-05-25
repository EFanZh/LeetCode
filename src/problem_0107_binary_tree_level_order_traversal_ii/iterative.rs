use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if let Some(node) = root {
            let mut queue = VecDeque::from(vec![node]);

            loop {
                let mut level = Vec::with_capacity(queue.len());

                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();

                    level.push(node.val);

                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }

                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    };
                }

                result.push(level);

                if queue.is_empty() {
                    break;
                }
            }

            result.reverse();
        }

        result
    }
}

impl super::Solution for Solution {
    fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        Self::level_order_bottom(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
