use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn count_dominant_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(5), Some(3), Some(8), Some(2), Some(4), Some(7), Some(1)] as &[_],
                5,
            ),
            (&[Some(1), Some(2), Some(3), Some(1), Some(2)], 4),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::count_dominant_nodes(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
