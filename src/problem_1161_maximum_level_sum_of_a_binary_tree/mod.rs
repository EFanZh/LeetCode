use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), Some(7), Some(0), Some(7), Some(-8), None, None] as &[_], 2),
            (
                &[
                    Some(989),
                    None,
                    Some(10250),
                    Some(98693),
                    Some(-89388),
                    None,
                    None,
                    None,
                    Some(-32127),
                ],
                2,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::max_level_sum(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
