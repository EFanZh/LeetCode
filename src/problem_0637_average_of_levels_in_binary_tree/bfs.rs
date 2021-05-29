use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    #[allow(clippy::cast_precision_loss)]
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = Vec::new();

        if let Some(node) = root {
            let mut queue = VecDeque::from(vec![node]);

            loop {
                let mut sum = 0_i64;
                let length = queue.len();

                for _ in 0..length {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();

                    sum += i64::from(node.val);

                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }

                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    };
                }

                result.push((sum as f64) / (length as f64));

                if queue.is_empty() {
                    break;
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        Self::average_of_levels(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
