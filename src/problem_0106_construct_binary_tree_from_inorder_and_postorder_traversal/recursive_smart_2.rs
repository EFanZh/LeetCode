use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn build_tree_helper_right(
        inorder: &mut &[i32],
        postorder: &mut &[i32],
        root_val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (&last_inorder, rest_inorder_0) = inorder.split_last().unwrap();

        if last_inorder == root_val {
            *inorder = rest_inorder_0;

            None
        } else {
            let (&val, rest_postorder) = postorder.split_last().unwrap();

            *postorder = rest_postorder;

            let right = Self::build_tree_helper_right(inorder, postorder, val);
            let left = Self::build_tree_helper_right(inorder, postorder, root_val);

            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
    }

    fn build_tree_helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        postorder.split_last().map(|(&val, rest_postorder)| {
            let mut inorder = inorder;
            let mut postorder = rest_postorder;
            let right = Self::build_tree_helper_right(&mut inorder, &mut postorder, val);
            let left = Self::build_tree_helper(inorder, postorder);

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
