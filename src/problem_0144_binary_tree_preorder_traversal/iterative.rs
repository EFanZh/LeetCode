use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn preorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut cont = Vec::new();

        loop {
            if let Some(node) = root {
                let node_ref = node.borrow();

                result.push(node_ref.val);
                cont.push(node_ref.right.clone());

                root = node_ref.left.clone();
            } else if let Some(right) = cont.pop() {
                root = right;
            } else {
                break;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::preorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}
