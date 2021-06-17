use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;
pub mod recursive_cached;
pub mod recursive_smart;
pub mod recursive_smart_2;

pub trait Solution {
    fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            (&[3, 9, 20, 15, 7] as &[_], &[9, 3, 15, 20, 7] as &[_]),
            &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] as &[_],
        )];

        for ((preorder, inorder), expected) in test_cases {
            assert_eq!(
                S::build_tree(preorder.to_vec(), inorder.to_vec()),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
