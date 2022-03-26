use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    fn find_node(
        queue: &VecDeque<(Rc<RefCell<TreeNode>>, i32)>,
        remaining: usize,
        value: i32,
        not_parent: i32,
    ) -> bool {
        for (node, parent) in queue.iter().take(remaining) {
            if node.borrow().val == value {
                return *parent != not_parent;
            }
        }

        false
    }

    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut queue = VecDeque::from(vec![(root.unwrap(), -1)]);

        loop {
            let mut remaining = queue.len();

            while remaining != 0 {
                let (node, parent) = queue.pop_front().unwrap();
                let node = node.borrow();

                remaining -= 1;

                if node.val == x {
                    return Self::find_node(&queue, remaining, y, parent);
                } else if node.val == y {
                    return Self::find_node(&queue, remaining, x, parent);
                }

                if let Some(left) = node.left.clone() {
                    queue.push_back((left, node.val));
                }

                if let Some(right) = node.right.clone() {
                    queue.push_back((right, node.val));
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        Self::is_cousins(root, x, y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
