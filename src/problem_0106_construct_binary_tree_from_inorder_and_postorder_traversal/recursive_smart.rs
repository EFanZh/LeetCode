use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn build_tree_helper_right<'a, 'b>(
        inorder: &'a [i32],
        postorder: &'b [i32],
        root_val: i32,
    ) -> (Tree, &'a [i32], &'b [i32]) {
        let (&last_inorder, rest_inorder_0) = inorder.split_last().unwrap();

        if last_inorder == root_val {
            (None, rest_inorder_0, postorder)
        } else {
            let (&val, rest_postorder) = postorder.split_last().unwrap();
            let (right, rest_inorder, rest_postorder) = Self::build_tree_helper_right(inorder, rest_postorder, val);

            let (left, rest_inorder, rest_postorder) =
                Self::build_tree_helper_right(rest_inorder, rest_postorder, root_val);

            (
                Some(Rc::new(RefCell::new(TreeNode { val, left, right }))),
                rest_inorder,
                rest_postorder,
            )
        }
    }

    fn build_tree_helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        postorder.split_last().map(|(&val, rest_postorder)| {
            let (right, rest_inorder, rest_postorder) = Self::build_tree_helper_right(inorder, rest_postorder, val);
            let left = Self::build_tree_helper(rest_inorder, rest_postorder);

            Rc::new(RefCell::new(TreeNode { val, left, right }))
        })
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(&inorder, &postorder)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
