use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut node = root.unwrap();
        let mut queue = VecDeque::new();

        loop {
            let node_ref = node.borrow();

            if let Some(right) = node_ref.right.clone() {
                queue.push_back(right);
            }

            if let Some(left) = node_ref.left.clone() {
                queue.push_back(left);
            }

            if let Some(next) = queue.pop_front() {
                drop(node_ref);

                node = next;
            } else {
                return node_ref.val;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
