use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;
pub mod recursive_2;

pub trait Solution {
    fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), Some(2), Some(3), Some(4), Some(5)] as &[_], 3),
            (&[], 0),
            (&[Some(1), Some(2)], 1),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::diameter_of_binary_tree(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
