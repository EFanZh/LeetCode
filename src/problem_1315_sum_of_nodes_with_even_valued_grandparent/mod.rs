use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;
pub mod recursive_2;

pub trait Solution {
    fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
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
                ] as &[_],
                18,
            ),
            (&[Some(1)], 0),
            (
                &[Some(50), None, Some(54), Some(98), Some(6), None, None, None, Some(34)],
                138,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::sum_even_grandparent(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
