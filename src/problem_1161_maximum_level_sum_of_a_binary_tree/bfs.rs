use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::from(vec![root.unwrap()]);
        let mut max_sum = i32::MIN;
        let mut max_level = 0;
        let mut level = 1;

        loop {
            let mut sum = 0;

            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();

                sum += node.val;

                if let Some(left) = node.left.clone() {
                    queue.push_back(left);
                }

                if let Some(right) = node.right.clone() {
                    queue.push_back(right);
                }
            }

            if sum > max_sum {
                max_sum = sum;
                max_level = level;
            }

            if queue.is_empty() {
                break;
            }

            level += 1;
        }

        max_level
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_level_sum(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
