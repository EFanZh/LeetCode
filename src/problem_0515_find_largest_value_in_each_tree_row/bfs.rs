use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(node) = root {
            let mut queue = VecDeque::from(vec![node]);

            loop {
                let mut max_value = i32::MIN;

                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();

                    max_value = max_value.max(node.val);

                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }

                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    };
                }

                result.push(max_value);

                if queue.is_empty() {
                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::largest_values(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
