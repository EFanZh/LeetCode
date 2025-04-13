use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(mut node) = root {
            let mut stack = Vec::new();

            'outer: loop {
                loop {
                    let left = node.borrow().left.clone();

                    if let Some(left) = left {
                        stack.push(node);

                        node = left;
                    } else {
                        break;
                    }
                }

                loop {
                    let (val, right) = {
                        let node = node.borrow();

                        (node.val, node.right.clone())
                    };

                    result.push(val);

                    if let Some(right) = right {
                        node = right;

                        break;
                    }

                    if let Some(top) = stack.pop() {
                        node = top;
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
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::inorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
