use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;

pub trait Solution {
    fn insert_into_max_tree(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[Some(4), Some(1), Some(3), None, None, Some(2)] as &[_], 5),
                &[Some(5), Some(4), None, Some(1), Some(3), None, None, Some(2)] as &[_],
            ),
            (
                (&[Some(5), Some(2), Some(4), None, Some(1)], 3),
                &[Some(5), Some(2), Some(4), None, Some(1), None, Some(3)],
            ),
            (
                (&[Some(5), Some(2), Some(3), None, Some(1)], 4),
                &[Some(5), Some(2), Some(4), None, Some(1), Some(3)],
            ),
        ];

        for ((root, val), expected) in test_cases {
            assert_eq!(
                S::insert_into_max_tree(test_utilities::make_tree(root.iter().copied()), val),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
