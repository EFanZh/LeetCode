use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod stack;

pub trait Solution {
    fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[3, 2, 1, 6, 0, 5] as &[_],
                &[Some(6), Some(3), Some(5), None, Some(2), Some(0), None, None, Some(1)] as &[_],
            ),
            (&[3, 2, 1], &[Some(3), None, Some(2), None, Some(1)]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(
                S::construct_maximum_binary_tree(nums.to_vec()),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
