use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(mut current) = root {
            loop {
                let mut current_ref = current.borrow_mut();

                if let Some(left) = current_ref.left.take() {
                    drop(current_ref);

                    let mut node = Rc::clone(&left);

                    loop {
                        let mut node_ref = node.borrow_mut();

                        if let Some(right) = node_ref.right.clone() {
                            drop(node_ref);
                            node = right;
                        } else {
                            node_ref.right = Some(mem::replace(&mut current, left));

                            break;
                        }
                    }
                } else {
                    result.push(current_ref.val);

                    if let Some(right) = current_ref.right.clone() {
                        drop(current_ref);

                        current = right;
                    } else {
                        break;
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
