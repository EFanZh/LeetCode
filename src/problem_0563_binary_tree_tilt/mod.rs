use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), Some(2), Some(3)] as &[_], 1),
            (&[Some(4), Some(2), Some(9), Some(3), Some(5), None, Some(7)], 15),
            (
                &[
                    Some(21),
                    Some(7),
                    Some(14),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                ],
                9,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(S::find_tilt(test_utilities::make_tree(root.iter().copied())), expected);
        }
    }
}
