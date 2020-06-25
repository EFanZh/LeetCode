use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    fn build_tree_helper(
        inorder: &[i32],
        postorder: &[i32],
        inorder_base: usize,
        inorder_val_to_index: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        postorder.split_last().map(|(&val, rest_postorder)| {
            let raw_split = inorder_val_to_index[&val];
            let split = raw_split - inorder_base;

            let left = Self::build_tree_helper(
                &inorder[..split],
                &rest_postorder[..split],
                inorder_base,
                inorder_val_to_index,
            );

            let right = Self::build_tree_helper(
                &inorder[split + 1..],
                &rest_postorder[split..],
                raw_split + 1,
                inorder_val_to_index,
            );

            Rc::new(RefCell::new(TreeNode { val, left, right }))
        })
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(
            &inorder,
            &postorder,
            0,
            &inorder.iter().copied().enumerate().map(|(val, i)| (i, val)).collect(),
        )
    }
}

impl super::Solution for Solution {
    fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree(inorder, postorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
