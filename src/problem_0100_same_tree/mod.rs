use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod cheating;
pub mod recursive;

pub trait Solution {
    fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[Some(1), Some(2), Some(3)] as &[_],
                    &[Some(1), Some(2), Some(3)] as &[_],
                ),
                true,
            ),
            ((&[Some(1), Some(2)], &[Some(1), None, Some(2)]), false),
        ];

        for ((p, q), expected) in test_cases {
            assert_eq!(
                S::is_same_tree(
                    test_utilities::make_tree(p.iter().copied()),
                    test_utilities::make_tree(q.iter().copied())
                ),
                expected
            );
        }
    }
}
