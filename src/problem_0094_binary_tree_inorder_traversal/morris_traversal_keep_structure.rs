use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(mut current) = root {
            'outer: loop {
                let mut current_ref = current.borrow_mut();

                if let Some(left) = current_ref.left.take() {
                    let mut node = left.clone();

                    loop {
                        let mut node_ref = node.borrow_mut();

                        if let Some(right) = node_ref.right.clone() {
                            if Rc::ptr_eq(&right, &current) {
                                node_ref.right = None;

                                break;
                            } else {
                                drop(node_ref);
                                node = right;
                            }
                        } else {
                            drop(current_ref);
                            node_ref.right = Some(mem::replace(&mut current, left));

                            continue 'outer;
                        }
                    }
                }

                result.push(current_ref.val);

                if let Some(right) = current_ref.right.clone() {
                    drop(current_ref);

                    current = right;
                } else {
                    break;
                }
            }
        }

        result
    }
}

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
