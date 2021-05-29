use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(node) = root {
            let mut queue = VecDeque::from(vec![node]);

            loop {
                for _ in 1..queue.len() {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();

                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }

                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }
                }

                let node = queue.pop_front().unwrap();
                let node = node.borrow();

                result.push(node.val);

                if let Some(left) = node.left.clone() {
                    queue.push_back(left);
                }

                if let Some(right) = node.right.clone() {
                    queue.push_back(right);
                }

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
