use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[Some(1), Some(3), Some(2), Some(5)] as &[_],
                    &[Some(2), Some(1), Some(3), None, Some(4), None, Some(7)] as &[_],
                ),
                &[Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)] as &[_],
            ),
            ((&[Some(1)], &[Some(1), Some(2)]), &[Some(2), Some(2)]),
        ];

        for ((root1, root2), expected) in test_cases {
            assert_eq!(
                S::merge_trees(
                    test_utilities::make_tree(root1.iter().copied()),
                    test_utilities::make_tree(root2.iter().copied())
                ),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
