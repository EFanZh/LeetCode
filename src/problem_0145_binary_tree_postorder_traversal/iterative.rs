use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(mut node) = root {
            let mut stack = Vec::new();

            'outer: loop {
                let mut val = loop {
                    let (val, left, right) = {
                        let node = node.borrow();

                        (node.val, node.left.clone(), node.right.clone())
                    };

                    let (next, right) = match (left, right) {
                        (None, None) => break val,
                        (None, Some(child)) | (Some(child), None) => (child, None),
                        (Some(left), Some(right)) => (left, Some(right)),
                    };

                    stack.push((val, right));

                    node = next;
                };

                loop {
                    result.push(val);

                    if let Some(top) = stack.last_mut() {
                        if let Some(right) = top.1.take() {
                            node = right;

                            break;
                        }

                        val = top.0;

                        stack.pop();
                    } else {
                        break 'outer;
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
