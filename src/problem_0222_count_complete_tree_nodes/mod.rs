use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)] as &[_], 6),
            (&[Some(1), Some(2), Some(3)], 3),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::count_nodes(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
