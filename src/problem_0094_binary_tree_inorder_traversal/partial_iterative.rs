use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn inorder_traversal_helper(mut root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        while let Some(node) = root {
            let node_ref = node.borrow();

            Self::inorder_traversal_helper(node_ref.left.clone(), result);
            result.push(node_ref.val);

            root = node_ref.right.clone();
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        Self::inorder_traversal_helper(root, &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::inorder_traversal(root)
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
