use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 3, 5, 8, 13, 21, 34] as &[_], &[2, 5, 3, 8, 13, 21, 34] as &[_]),
            (&[7, 13, 11], &[7, 11, 13]),
            (
                &[0, 1, 2, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2],
                &[0, 2, 1, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 1, 1],
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::reverse_odd_levels(test_utilities::make_tree(root.iter().copied().map(Some))),
                test_utilities::make_tree(expected.iter().copied().map(Some)),
            );
        }
    }
}
