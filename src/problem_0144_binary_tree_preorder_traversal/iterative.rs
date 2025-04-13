use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(mut node) = root {
            let mut stack = Vec::new();

            loop {
                loop {
                    let (val, left, right) = {
                        let node = node.borrow();

                        (node.val, node.left.clone(), node.right.clone())
                    };

                    result.push(val);

                    node = if let Some(left) = left {
                        if let Some(right) = right {
                            stack.push(right);
                        }

                        left
                    } else if let Some(right) = right {
                        right
                    } else {
                        break;
                    };
                }

                if let Some(top) = stack.pop() {
                    node = top;
                } else {
                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::preorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
