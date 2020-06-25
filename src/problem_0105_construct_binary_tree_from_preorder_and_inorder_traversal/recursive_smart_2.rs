use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn build_tree_helper_left(
        preorder: &mut &[i32],
        inorder: &mut &[i32],
        root_val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (&first_inorder, rest_inorder_0) = inorder.split_first().unwrap();

        if first_inorder == root_val {
            *inorder = rest_inorder_0;

            None
        } else {
            let (&val, rest_preorder) = preorder.split_first().unwrap();

            *preorder = rest_preorder;

            let left = Self::build_tree_helper_left(preorder, inorder, val);
            let right = Self::build_tree_helper_left(preorder, inorder, root_val);

            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
    }

    fn build_tree_helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        preorder.split_first().map(|(&val, rest_preorder)| {
            let mut preorder = rest_preorder;
            let mut inorder = inorder;
            let left = Self::build_tree_helper_left(&mut preorder, &mut inorder, val);
            let right = Self::build_tree_helper(preorder, inorder);

            Rc::new(RefCell::new(TreeNode { val, left, right }))
        })
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(&preorder, &inorder)
    }
}

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
