use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)] as &[_],
                &[1, 3, 9] as &[_],
            ),
            (&[Some(1), Some(2), Some(3)], &[1, 3]),
            (&[Some(1)], &[1]),
            (&[Some(1), None, Some(2)], &[1, 2]),
            (&[], &[]),
        ];

        for (root, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::largest_values(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
