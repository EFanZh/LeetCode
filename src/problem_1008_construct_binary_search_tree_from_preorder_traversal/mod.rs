use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[8, 5, 1, 7, 10, 12] as &[_],
                &[Some(8), Some(5), Some(10), Some(1), Some(7), None, Some(12)] as &[_],
            ),
            (&[1, 3], &[Some(1), None, Some(3)]),
        ];

        for (preorder, expected) in test_cases {
            assert_eq!(
                S::bst_from_preorder(preorder.to_vec()),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
