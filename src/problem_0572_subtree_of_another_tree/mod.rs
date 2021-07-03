use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod hashing;

pub trait Solution {
    fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[Some(3), Some(4), Some(5), Some(1), Some(2)] as &[_],
                    &[Some(4), Some(1), Some(2)] as &[_],
                ),
                true,
            ),
            (
                (
                    &[Some(3), Some(4), Some(5), Some(1), Some(2), None, None, None, Some(0)],
                    &[Some(4), Some(1), Some(2)],
                ),
                false,
            ),
        ];

        for ((s, t), expected) in test_cases {
            assert_eq!(
                S::is_subtree(
                    test_utilities::make_tree(s.iter().copied()),
                    test_utilities::make_tree(t.iter().copied())
                ),
                expected
            );
        }
    }
}
