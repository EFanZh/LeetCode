use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn postorder_traversal_helper(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let node_ref = node.borrow();

            Self::postorder_traversal_helper(&node_ref.left, result);
            Self::postorder_traversal_helper(&node_ref.right, result);

            result.push(node_ref.val);
        }
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        Self::postorder_traversal_helper(&root, &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::postorder_traversal(root)
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
