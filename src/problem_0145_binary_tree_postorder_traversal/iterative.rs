use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

enum Frame {
    Left(Rc<RefCell<TreeNode>>),
    Right(Rc<RefCell<TreeNode>>),
}

impl Solution {
    pub fn postorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut cont = Vec::new();

        'r: loop {
            if let Some(node) = root {
                root = node.borrow().left.clone();

                cont.push(Frame::Left(node));
            } else {
                // Apply continuation.

                'k: loop {
                    match cont.pop() {
                        Some(Frame::Left(node)) => {
                            // Left continuation.

                            root = node.borrow().right.clone();

                            cont.push(Frame::Right(node));

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

impl super::Solution for Solution {
    fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::postorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}
