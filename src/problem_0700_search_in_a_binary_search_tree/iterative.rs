use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = root;

        while let Some(node_rc) = node {
            let node_ref = node_rc.borrow();

            node = match val.cmp(&node_ref.val) {
                Ordering::Less => node_ref.left.clone(),
                Ordering::Equal => {
                    drop(node_ref);

                    return Some(node_rc);
                }
                Ordering::Greater => node_ref.right.clone(),
            };
        }

        None
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::search_bst(root, val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
