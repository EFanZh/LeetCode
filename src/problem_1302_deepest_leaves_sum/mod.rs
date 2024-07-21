use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    None,
                    Some(6),
                    Some(7),
                    None,
                    None,
                    None,
                    None,
                    Some(8),
                ] as &[_],
                15,
            ),
            (
                &[
                    Some(6),
                    Some(7),
                    Some(8),
                    Some(2),
                    Some(7),
                    Some(1),
                    Some(3),
                    Some(9),
                    None,
                    Some(1),
                    Some(4),
                    None,
                    None,
                    None,
                    Some(5),
                ],
                19,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::deepest_leaves_sum(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
