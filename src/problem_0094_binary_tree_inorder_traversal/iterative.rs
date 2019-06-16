use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut cont = Vec::new();

        'k: loop {
            if let Some(node) = root {
                root = node.borrow().left.clone();

                cont.push(Some(node));
            } else {
                // Apply continuation.

                loop {
                    match cont.pop() {
                        Some(Some(node)) => {
                            // Left continuation.

                            // Should I use `Ref::map_split`?

                            result.push(node.borrow().val);

                            root = node.borrow().right.clone();
                            cont.push(None);

                            continue 'k;
                        }
                        Some(None) => {
                            // Right continuation.
                        }
                        None => {
                            // Root continuation.

                            break 'k;
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
