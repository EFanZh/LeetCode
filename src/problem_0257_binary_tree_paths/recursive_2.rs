use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn binary_tree_paths_helper(node: &TreeNode, base: &mut String, result: &mut Vec<String>) {
        use std::fmt::Write;

        let saved_len = base.len();

        write!(base, "{}", node.val).unwrap();

        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => result.push(base.clone()),
            (None, Some(child)) | (Some(child), None) => {
                base.push_str("->");

                Self::binary_tree_paths_helper(&child.borrow(), base, result);
            }
            (Some(left), Some(right)) => {
                base.push_str("->");

                Self::binary_tree_paths_helper(&left.borrow(), base, result);
                Self::binary_tree_paths_helper(&right.borrow(), base, result);
            }
        }

        base.truncate(saved_len);
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = Vec::new();

        if let Some(node) = root.as_deref() {
            Self::binary_tree_paths_helper(&node.borrow(), &mut String::new(), &mut result);
        }

        result
    }
}

impl super::Solution for Solution {
    fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        Self::binary_tree_paths(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
