use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[Some(3), Some(1), Some(4), None, Some(2)] as &[_], 1), 1),
            (
                (&[Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)], 3),
                3,
            ),
        ];

        for ((root, k), expected) in test_cases {
            assert_eq!(
                S::kth_smallest(test_utilities::make_tree(root.iter().copied()), k),
                expected
            );
        }
    }
}
