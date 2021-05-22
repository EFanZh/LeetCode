use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn new_node_with_left_child(val: i32, node: Option<Rc<RefCell<TreeNode>>>) -> TreeNode {
        TreeNode {
            val,
            left: node,
            right: None,
        }
    }

    fn new_node_with_right_child(val: i32, node: Option<Rc<RefCell<TreeNode>>>) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: node,
        }
    }

    fn helper(
        node: &mut Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
        make_child: impl FnOnce(i32, Option<Rc<RefCell<TreeNode>>>) -> TreeNode,
    ) {
        if depth == 1 {
            *node = Some(Rc::new(RefCell::new(make_child(val, node.take()))));
        } else if let Some(mut node) = node.as_deref().map(RefCell::borrow_mut) {
            Self::helper(&mut node.left, val, depth - 1, Self::new_node_with_left_child);
            Self::helper(&mut node.right, val, depth - 1, Self::new_node_with_right_child);
        }
    }

    pub fn add_one_row(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&mut root, val, depth, Self::new_node_with_left_child);

        root
    }
}

impl super::Solution for Solution {
    fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::add_one_row(root, val, depth)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
