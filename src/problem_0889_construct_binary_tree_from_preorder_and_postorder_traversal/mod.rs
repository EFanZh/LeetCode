use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;
pub mod recursive_2;

pub trait Solution {
    fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 4, 5, 3, 6, 7] as &[_], &[4, 5, 2, 6, 7, 3, 1] as &[_]),
            (&[1], &[1]),
            (
                &[1, 2, 4, 5, 8, 3, 6, 9, 7, 10, 11],
                &[4, 8, 5, 2, 9, 6, 10, 11, 7, 3, 1],
            ),
        ];

        for (preorder, postorder) in test_cases {
            let result = S::construct_from_pre_post(preorder.to_vec(), postorder.to_vec());
            let pre_order_result = test_utilities::iter_tree_pre_order(result.clone()).collect::<Vec<_>>();
            let post_order_result = test_utilities::iter_tree_post_order(result).collect::<Vec<_>>();

            assert_eq!(pre_order_result, preorder);
            assert_eq!(post_order_result, postorder);
        }
    }
}
