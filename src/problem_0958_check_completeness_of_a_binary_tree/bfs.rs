use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.map_or(true, |mut node| {
            let mut queue = VecDeque::new();

            loop {
                let node_ref = node.borrow();

                if let Some(left) = node_ref.left.clone() {
                    queue.push_back(left);

                    if let Some(right) = node_ref.right.clone() {
                        queue.push_back(right);
                    } else {
                        break;
                    }
                } else {
                    if node_ref.right.is_some() {
                        return false;
                    }

                    break;
                }

                drop(node_ref);

                node = queue.pop_front().unwrap();
            }

            queue.iter().all(|node| {
                let node_ref = node.borrow();

                node_ref.left.is_none() && node_ref.right.is_none()
            })
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_complete_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
