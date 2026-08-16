use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(5), Some(4), Some(9), Some(1), Some(10), None, Some(7)] as &[_],
                &[Some(0), Some(0), Some(0), Some(7), Some(7), None, Some(11)] as &[_],
            ),
            (&[Some(3), Some(1), Some(2)], &[Some(0), Some(0), Some(0)]),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::replace_value_in_tree(test_utilities::make_tree(root.iter().copied())),
                test_utilities::make_tree(expected.iter().copied()),
            );
        }
    }
}
