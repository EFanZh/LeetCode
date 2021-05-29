use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter;
use std::rc::Rc;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if let Some(node) = root {
            let mut queue = iter::once(node).collect::<VecDeque<_>>();

            loop {
                {
                    let mut level = Vec::with_capacity(queue.len());

                    for _ in 0..queue.len() {
                        let node = queue.pop_front().unwrap();
                        let node = node.borrow();

                        level.push(node.val);

                        if let Some(left) = node.left.clone() {
                            queue.push_back(left);
                        }

                        if let Some(right) = node.right.clone() {
                            queue.push_back(right);
                        }
                    }

                    result.push(level);
                }

                if queue.is_empty() {
                    break;
                }

                {
                    let mut level = Vec::with_capacity(queue.len());

                    for _ in 0..queue.len() {
                        let node = queue.pop_back().unwrap();
                        let node = node.borrow();

                        level.push(node.val);

                        if let Some(right) = node.right.clone() {
                            queue.push_front(right);
                        }

                        if let Some(left) = node.left.clone() {
                            queue.push_front(left);
                        }
                    }

                    result.push(level);
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
    fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        Self::zigzag_level_order(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
