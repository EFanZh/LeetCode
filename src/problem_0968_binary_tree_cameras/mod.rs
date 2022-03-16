use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod dynamic_programming;

pub trait Solution {
    fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(0), Some(0), None, Some(0), Some(0)] as &[_], 1),
            (
                &[Some(0), Some(0), None, Some(0), None, Some(0), None, None, Some(0)],
                2,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::min_camera_cover(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
