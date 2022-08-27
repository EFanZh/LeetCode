use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::from(vec![root.unwrap()]);

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

            if queue.is_empty() {
                return sum;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::deepest_leaves_sum(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
