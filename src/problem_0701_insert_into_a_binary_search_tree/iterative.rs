use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let new_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));

        if let Some(mut node) = root.clone() {
            loop {
                let mut node_ref = node.borrow_mut();

                if val < node_ref.val {
                    if let Some(left) = node_ref.left.clone() {
                        drop(node_ref);

                        node = left;
                    } else {
                        node_ref.left = new_node;

                        break;
                    }
                } else if let Some(right) = node_ref.right.clone() {
                    drop(node_ref);

                    node = right;
                } else {
                    node_ref.right = new_node;

                    break;
                }
            }

            root
        } else {
            new_node
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::insert_into_bst(root, val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
