use super::super::data_structures::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    fn helper(root: &mut Option<Rc<RefCell<TreeNode>>>, key: i32) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            match key.cmp(&node.val) {
                Ordering::Less => Self::helper(&mut node.left, key),
                Ordering::Equal => {
                    let children = (node.left.take(), node.right.take());

                    drop(node);

                    match children {
                        (None, None) => *root = None,
                        (None, Some(child)) | (Some(child), None) => *root = Some(child),
                        (Some(left), Some(right)) => {
                            let mut right_ref = right.borrow_mut();

                            if let Some(right_left) = &right_ref.left {
                                let mut right_min_parent = right.clone();
                                let mut right_min = right_left.clone();

                                drop(right_ref);

                                loop {
                                    let mut right_min_ref = right_min.borrow_mut();
                                    let right_min_left = right_min_ref.left.clone();

                                    if let Some(right_min_left) = right_min_left {
                                        drop(right_min_ref);
                                        right_min_parent = right_min;
                                        right_min = right_min_left;
                                    } else {
                                        right_min_parent.borrow_mut().left = right_min_ref.right.take();
                                        right_min_ref.left = Some(left);
                                        right_min_ref.right = Some(right);

                                        break;
                                    }
                                }

                                *root = Some(right_min);
                            } else {
                                right_ref.left = Some(left);
                                drop(right_ref);
                                *root = Some(right);
                            }
                        }
                    }
                }
                Ordering::Greater => Self::helper(&mut node.right, key),
            }
        }
    }

    pub fn delete_node(mut root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&mut root, key);

        root
    }
}

impl super::Solution for Solution {
    fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::delete_node(root, key)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
