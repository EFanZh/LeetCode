use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod merge_iterators;

pub trait Solution {
    fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[Some(2), Some(1), Some(4)] as &[_],
                    &[Some(1), Some(0), Some(3)] as &[_],
                ),
                &[0, 1, 1, 2, 3, 4] as &[_],
            ),
            ((&[Some(1), None, Some(8)], &[Some(8), Some(1)]), &[1, 1, 8, 8]),
            ((&[], &[]), &[]),
        ];

        for ((root1, root2), expected) in test_cases {
            assert_eq!(
                S::get_all_elements(
                    test_utilities::make_tree(root1.iter().copied()),
                    test_utilities::make_tree(root2.iter().copied())
                ),
                expected
            );
        }
    }
}
