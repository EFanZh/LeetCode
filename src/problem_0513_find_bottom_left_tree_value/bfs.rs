use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::from(vec![root.unwrap()]);

        loop {
            let first_value = queue.front().unwrap().borrow().val;

            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();

                if let Some(left) = node.left.clone() {
                    queue.push_back(left);
                }

                if let Some(right) = node.right.clone() {
                    queue.push_back(right);
                }
            }

            if queue.is_empty() {
                return first_value;
            }
        }
    }
}

impl super::Solution for Solution {
    fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find_bottom_left_value(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
