use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    #[allow(clippy::unnecessary_wraps)] // Expected.
    pub fn insert_into_max_tree(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root.unwrap();

        if val < root.borrow().val {
            let mut parent = Rc::clone(&root);

            loop {
                let mut parent_ref = parent.borrow_mut();

                if let Some(right) = parent_ref.right.as_ref() {
                    if val < right.borrow().val {
                        let new_parent = Rc::clone(right);

                        drop(parent_ref);

                        parent = new_parent;
                    } else {
                        let right = parent_ref.right.take();

                        parent_ref.right = Some(Rc::new(RefCell::new(TreeNode {
                            val,
                            left: right,
                            right: None,
                        })));

                        break;
                    }
                } else {
                    parent_ref.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));

                    break;
                }
            }

            Some(root)
        } else {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Some(root),
                right: None,
            })))
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn insert_into_max_tree(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::insert_into_max_tree(root, val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
