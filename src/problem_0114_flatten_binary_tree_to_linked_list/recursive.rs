use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn flatten_helper(tail: &mut Rc<RefCell<TreeNode>>, node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let new_tail = tail.borrow_mut().right.get_or_insert(node).clone();

            *tail = new_tail;

            let (left, right) = {
                let mut tail = tail.borrow_mut();

                (tail.left.take(), tail.right.take())
            };

            Self::flatten_helper(tail, left);
            Self::flatten_helper(tail, right);
        }
    }

    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(mut node) = root.clone() {
            let (left, right) = {
                let mut node = node.borrow_mut();

                (node.left.take(), node.right.take())
            };

            Self::flatten_helper(&mut node, left);
            Self::flatten_helper(&mut node, right);
        }
    }
}

impl super::Solution for Solution {
    fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::flatten(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
