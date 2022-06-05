use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

impl Solution {
    fn helper_non_root(
        node: &mut Option<Rc<RefCell<TreeNode>>>,
        to_delete: &mut HashSet<i32>,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) {
        if let Some(node_ref) = node.as_deref() {
            let mut node_ref = node_ref.borrow_mut();

            if to_delete.remove(&node_ref.val) {
                Self::helper_root(node_ref.left.take(), to_delete, result);
                Self::helper_root(node_ref.right.take(), to_delete, result);

                drop(node_ref);

                *node = None;
            } else {
                Self::helper_non_root(&mut node_ref.left, to_delete, result);
                Self::helper_non_root(&mut node_ref.right, to_delete, result);
            }
        }
    }

    fn helper_root(
        node: Option<Rc<RefCell<TreeNode>>>,
        to_delete: &mut HashSet<i32>,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) {
        if let Some(node) = node {
            let mut node_ref = node.borrow_mut();

            if to_delete.remove(&node_ref.val) {
                Self::helper_root(node_ref.left.take(), to_delete, result);
                Self::helper_root(node_ref.right.take(), to_delete, result);
            } else {
                Self::helper_non_root(&mut node_ref.left, to_delete, result);
                Self::helper_non_root(&mut node_ref.right, to_delete, result);

                drop(node_ref);

                result.push(Some(node));
            }
        }
    }

    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut to_delete = HashSet::from_iter(to_delete);
        let mut result = Vec::new();

        Self::helper_root(root, &mut to_delete, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::del_nodes(root, to_delete)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
