use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    #[allow(clippy::too_many_lines)]
    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[Some(1), Some(2), Some(3), Some(4)] as &[_], 4, 3), false),
            ((&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5)], 5, 4), true),
            ((&[Some(1), Some(2), Some(3), None, Some(4)], 2, 3), false),
            (
                (
                    &[Some(1), Some(3), Some(2), None, None, Some(5), Some(4), Some(6)],
                    5,
                    6,
                ),
                false,
            ),
            (
                (
                    &[Some(1), None, Some(2), Some(3), Some(4), None, Some(6), None, Some(5)],
                    5,
                    3,
                ),
                false,
            ),
        ];

        for ((root, x, y), expected) in test_cases {
            assert_eq!(
                S::is_cousins(test_utilities::make_tree(root.iter().copied()), x, y),
                expected
            );
        }
    }
}
