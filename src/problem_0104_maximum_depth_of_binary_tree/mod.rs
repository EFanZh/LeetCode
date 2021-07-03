use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] as &[_], 3),
            (&[], 0),
            (&[Some(3)], 1),
        ];

        for (root, expected) in test_cases {
            assert_eq!(S::max_depth(test_utilities::make_tree(root.iter().copied())), expected);
        }
    }
}
