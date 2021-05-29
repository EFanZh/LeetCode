use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn build_tree_helper_left<'a, 'b>(
        preorder: &'a [i32],
        inorder: &'b [i32],
        root_val: i32,
    ) -> (Tree, &'a [i32], &'b [i32]) {
        let (&first_inorder, rest_inorder_0) = inorder.split_first().unwrap();

        if first_inorder == root_val {
            (None, preorder, rest_inorder_0)
        } else {
            let (&val, rest_preorder) = preorder.split_first().unwrap();
            let (left, rest_preorder, rest_inorder) = Self::build_tree_helper_left(rest_preorder, inorder, val);

            let (right, rest_preorder, rest_inorder) =
                Self::build_tree_helper_left(rest_preorder, rest_inorder, root_val);

            (
                Some(Rc::new(RefCell::new(TreeNode { val, left, right }))),
                rest_preorder,
                rest_inorder,
            )
        }
    }

    fn build_tree_helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        preorder.split_first().map(|(&val, rest_preorder)| {
            let (left, rest_preorder, rest_inorder) = Self::build_tree_helper_left(rest_preorder, inorder, val);
            let right = Self::build_tree_helper(rest_preorder, rest_inorder);

            Rc::new(RefCell::new(TreeNode { val, left, right }))
        })
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(&preorder, &inorder)
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
