use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::from(vec![(0, root.unwrap())]);
        let mut result = 0;

        loop {
            result = result.max(queue.back().unwrap().0 - queue.front().unwrap().0);

            for _ in 0..queue.len() {
                let (position, node) = queue.pop_front().unwrap();
                let node = node.borrow();

                if let Some(left) = node.left.clone() {
                    queue.push_back((position << 1, left));
                }

                if let Some(right) = node.right.clone() {
                    queue.push_back(((position << 1) + 1, right));
                }
            }

            if queue.is_empty() {
                break;
            }
        }

        result + 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::width_of_binary_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
