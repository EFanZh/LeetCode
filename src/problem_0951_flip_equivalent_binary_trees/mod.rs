use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool;
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
                        Some(1),
                        Some(2),
                        Some(3),
                        Some(4),
                        Some(5),
                        Some(6),
                        None,
                        None,
                        None,
                        Some(7),
                        Some(8),
                    ] as &[_],
                    &[
                        Some(1),
                        Some(3),
                        Some(2),
                        None,
                        Some(6),
                        Some(4),
                        Some(5),
                        None,
                        None,
                        None,
                        None,
                        Some(8),
                        Some(7),
                    ] as &[_],
                ),
                true,
            ),
            ((&[], &[]), true),
            ((&[], &[Some(1)]), false),
        ];

        for ((root1, root2), expected) in test_cases {
            assert_eq!(
                S::flip_equiv(
                    test_utilities::make_tree(root1.iter().copied()),
                    test_utilities::make_tree(root2.iter().copied())
                ),
                expected
            );
        }
    }
}
