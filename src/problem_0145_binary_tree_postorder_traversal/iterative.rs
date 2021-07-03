use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

enum Frame {
    Left(Rc<RefCell<TreeNode>>),
    Right(Rc<RefCell<TreeNode>>),
}

impl Solution {
    pub fn postorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        'r: loop {
            if let Some(node) = root {
                root = node.borrow().left.clone();

                stack.push(Frame::Left(node));
            } else {
                // Apply continuation.

                'k: loop {
                    match stack.pop() {
                        Some(Frame::Left(node)) => {
                            // Left continuation.

                            root = node.borrow().right.clone();

                            stack.push(Frame::Right(node));

                            continue 'r;
                        }
                        Some(Frame::Right(node)) => {
                            // Right continuation.

                            result.push(node.borrow().val);

                            continue 'k;
                        }
                        None => {
                            // Root continuation.

                            break 'r;
                        }
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::postorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
