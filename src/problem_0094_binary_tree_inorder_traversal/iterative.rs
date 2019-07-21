use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut cont = Vec::new();

        'r: loop {
            if let Some(node) = root {
                root = node.borrow().left.clone();

                cont.push(Some(node));
            } else {
                // Apply continuation.

                'k: loop {
                    match cont.pop() {
                        Some(Some(node)) => {
                            // Left continuation.

                            let node_ref = node.borrow();

                            result.push(node_ref.val);

                            root = node_ref.right.clone();
                            cont.push(None);

                            continue 'r;
                        }
                        Some(None) => {
                            // Right continuation.

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
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Solution::inorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
