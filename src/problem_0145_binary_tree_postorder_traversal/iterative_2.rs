use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::ptr::NonNull;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut prev: Option<NonNull<RefCell<TreeNode>>> = None;
        let mut root = root;

        'r: loop {
            if let Some(node) = root {
                root = node.borrow().left.clone();

                stack.push(node);
            } else {
                // Apply continuation.

                loop {
                    if let Some(node) = stack.last() {
                        let node_ref = node.borrow();

                        if node_ref.right.is_none()
                            || prev == node_ref.right.as_ref().map(|right| right.as_ref().into())
                        {
                            // Right continuation.

                            result.push(node_ref.val);

                            drop(node_ref);

                            prev = Some(node.as_ref().into());

                            stack.pop();
                        } else {
                            // Left continuation.

                            root = node_ref.right.clone();

                            break;
                        }
                    } else {
                        // Root continuation.

                        break 'r;
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
