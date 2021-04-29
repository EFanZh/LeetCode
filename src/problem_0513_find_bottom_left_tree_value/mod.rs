use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;
pub mod bfs_2;

pub trait Solution {
    fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(2), Some(1), Some(3)] as &[_], 1),
            (
                &[
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    None,
                    Some(5),
                    Some(6),
                    None,
                    None,
                    Some(7),
                ],
                7,
            ),
        ];

        for (root, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::find_bottom_left_value(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
