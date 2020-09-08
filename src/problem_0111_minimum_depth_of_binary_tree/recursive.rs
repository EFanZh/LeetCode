use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;

        if let Some(mut node) = root {
            let mut queue = VecDeque::new();

            depth = 1;

            loop {
                match {
                    let node_ref = node.borrow();

                    (node_ref.left.clone(), node_ref.right.clone())
                } {
                    (None, None) => break,
                    (None, Some(child)) | (Some(child), None) => {
                        if let Some((next, next_depth)) = queue.pop_front() {
                            queue.push_back((child, depth + 1));

                            node = next;
                            depth = next_depth;
                        } else {
                            node = child;
                            depth += 1;
                        }
                    }
                    (Some(left), Some(right)) => {
                        if let Some((next, next_depth)) = queue.pop_front() {
                            queue.push_back((left, depth + 1));
                            queue.push_back((right, depth + 1));

                            node = next;
                            depth = next_depth;
                        } else {
                            queue.push_back((right, depth + 1));

                            node = left;
                            depth += 1;
                        }
                    }
                }
            }
        }

        depth
    }
}

impl super::Solution for Solution {
    fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_depth(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
