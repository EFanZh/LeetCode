use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    fn build_tree_helper(
        preorder: &[i32],
        inorder: &[i32],
        inorder_base: usize,
        inorder_val_to_index: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        preorder.split_first().map(|(&val, rest_preorder)| {
            let raw_split = inorder_val_to_index[&val];
            let split = raw_split - inorder_base;

            let left = Self::build_tree_helper(
                &rest_preorder[..split],
                &inorder[..split],
                inorder_base,
                inorder_val_to_index,
            );

            let right = Self::build_tree_helper(
                &rest_preorder[split..],
                &inorder[split + 1..],
                raw_split + 1,
                inorder_val_to_index,
            );

            Rc::new(RefCell::new(TreeNode { val, left, right }))
        })
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(
            &preorder,
            &inorder,
            0,
            &inorder.iter().copied().enumerate().map(|(val, i)| (i, val)).collect(),
        )
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree(preorder, inorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
