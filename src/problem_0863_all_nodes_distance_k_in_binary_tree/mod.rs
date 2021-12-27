use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[
                        Some(3),
                        Some(5),
                        Some(1),
                        Some(6),
                        Some(2),
                        Some(0),
                        Some(8),
                        None,
                        None,
                        Some(7),
                        Some(4),
                    ] as &[_],
                    5,
                    2,
                ),
                &[1, 4, 7] as &[_],
            ),
            ((&[Some(1)], 1, 3), &[]),
            ((&[Some(0), Some(2), Some(1), None, None, Some(3)], 3, 3), &[2]),
            (
                (&[Some(0), None, Some(1), None, Some(2), None, Some(3), Some(4)], 2, 2),
                &[0, 4],
            ),
        ];

        for ((root, target, k), expected) in test_cases {
            let root = test_utilities::make_tree(root.iter().copied());
            let target = test_utilities::find_node(&root, target);

            assert_eq!(
                test_utilities::unstable_sorted(S::distance_k(root, target, k)),
                expected
            );
        }
    }
}
