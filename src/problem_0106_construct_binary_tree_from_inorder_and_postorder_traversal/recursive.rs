use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn build_tree_helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        postorder.split_last().map(|(&val, rest_postorder)| {
            let split = inorder.iter().position(|x| *x == val).unwrap();
            let left = Self::build_tree_helper(&inorder[..split], &rest_postorder[..split]);
            let right = Self::build_tree_helper(&inorder[split + 1..], &rest_postorder[split..]);

            Rc::new(RefCell::new(TreeNode { val, left, right }))
        })
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(&inorder, &postorder)
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
