use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] as &[_],
                &[&[9] as &[_], &[3, 15], &[20], &[7]] as &[&[_]],
            ),
            (
                &[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)],
                &[&[4], &[2], &[1, 5, 6], &[3], &[7]],
            ),
            (
                &[Some(1), Some(2), Some(3), Some(4), Some(6), Some(5), Some(7)],
                &[&[4], &[2], &[1, 5, 6], &[3], &[7]],
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::vertical_traversal(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
