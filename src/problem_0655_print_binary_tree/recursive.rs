use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn tree_height(node: &RefCell<TreeNode>) -> usize {
        let node = node.borrow();

        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => 1,
            (None, Some(child)) | (Some(child), None) => Self::tree_height(child) + 1,
            (Some(left), Some(right)) => Self::tree_height(left).max(Self::tree_height(right)) + 1,
        }
    }

    fn helper(node: &RefCell<TreeNode>, row: usize, start: usize, end: usize, result: &mut [Vec<String>]) {
        let middle = (start + end) / 2;
        let node = node.borrow();

        result[row][middle] = node.val.to_string();

        if let Some(left) = node.left.as_deref() {
            Self::helper(left, row + 1, start, middle, result);
        }

        if let Some(right) = node.right.as_deref() {
            Self::helper(right, row + 1, middle + 1, end, result);
        }
    }

    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let root = root.unwrap();
        let height = Self::tree_height(&root);
        let columns = (1 << height) - 1;
        let mut result = vec![vec![String::new(); columns]; height];

        Self::helper(&root, 0, 0, columns, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        Self::print_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
