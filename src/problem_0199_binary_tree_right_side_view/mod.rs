use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;
pub mod recursive;

pub trait Solution {
    fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(1), Some(2), Some(3), None, Some(5), None, Some(4)] as &[_],
                &[1, 3, 4] as &[_],
            ),
            (&[Some(1), None, Some(3)], &[1, 3]),
            (&[], &[]),
            (&[Some(1), Some(2), Some(3), Some(4)], &[1, 3, 4]),
        ];

        for (root, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::right_side_view(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
