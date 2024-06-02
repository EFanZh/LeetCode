use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(4), Some(8), Some(5), Some(0), Some(1), None, Some(6)] as &[_], 5),
            (&[Some(1)], 1),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::average_of_subtree(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
