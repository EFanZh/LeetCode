use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn trim_low(node_slot: &mut Option<Rc<RefCell<TreeNode>>>, low: i32) {
        while let Some(node) = node_slot {
            let mut node_ref = node.borrow_mut();

            if node_ref.val < low {
                let right = node_ref.right.take();

                drop(node_ref);

                *node_slot = right;
            } else {
                Self::trim_low(&mut node_ref.left, low);

                break;
            }
        }
    }

    fn trim_high(node_slot: &mut Option<Rc<RefCell<TreeNode>>>, high: i32) {
        while let Some(node) = node_slot {
            let mut node_ref = node.borrow_mut();

            if node_ref.val > high {
                let left = node_ref.left.take();

                drop(node_ref);

                *node_slot = left;
            } else {
                Self::trim_high(&mut node_ref.right, high);

                break;
            }
        }
    }

    fn trim(node_slot: &mut Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) {
        while let Some(node) = node_slot {
            let mut node_ref = node.borrow_mut();

            if node_ref.val < low {
                let right = node_ref.right.take();

                drop(node_ref);

                *node_slot = right;
            } else if node_ref.val <= high {
                Self::trim_low(&mut node_ref.left, low);
                Self::trim_high(&mut node_ref.right, high);

                break;
            } else {
                let left = node_ref.left.take();

                drop(node_ref);

                *node_slot = left;
            }
        }
    }

    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;

        Self::trim(&mut root, low, high);

        root
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::trim_bst(root, low, high)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
