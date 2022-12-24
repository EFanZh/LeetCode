use crate::data_structures::{ListNode, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

pub mod kmp;
pub mod kmp_2;

pub trait Solution {
    fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[4, 2, 8] as &[_],
                    &[
                        Some(1),
                        Some(4),
                        Some(4),
                        None,
                        Some(2),
                        Some(2),
                        None,
                        Some(1),
                        None,
                        Some(6),
                        Some(8),
                        None,
                        None,
                        None,
                        None,
                        Some(1),
                        Some(3),
                    ] as &[_],
                ),
                true,
            ),
            (
                (
                    &[1, 4, 2, 6],
                    &[
                        Some(1),
                        Some(4),
                        Some(4),
                        None,
                        Some(2),
                        Some(2),
                        None,
                        Some(1),
                        None,
                        Some(6),
                        Some(8),
                        None,
                        None,
                        None,
                        None,
                        Some(1),
                        Some(3),
                    ],
                ),
                true,
            ),
            (
                (
                    &[1, 4, 2, 6, 8],
                    &[
                        Some(1),
                        Some(4),
                        Some(4),
                        None,
                        Some(2),
                        Some(2),
                        None,
                        Some(1),
                        None,
                        Some(6),
                        Some(8),
                        None,
                        None,
                        None,
                        None,
                        Some(1),
                        Some(3),
                    ],
                ),
                false,
            ),
            (
                (&[2, 2, 1], &[Some(2), None, Some(2), None, Some(2), None, Some(1)]),
                true,
            ),
        ];

        for ((head, root), expected) in test_cases {
            assert_eq!(
                S::is_sub_path(
                    test_utilities::make_list(head.iter().copied()),
                    test_utilities::make_tree(root.iter().copied()),
                ),
                expected,
            );
        }
    }
}
