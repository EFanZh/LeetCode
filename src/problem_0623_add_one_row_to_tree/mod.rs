use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[Some(4), Some(2), Some(6), Some(3), Some(1), Some(5)] as &[_], 1, 2),
                &[
                    Some(4),
                    Some(1),
                    Some(1),
                    Some(2),
                    None,
                    None,
                    Some(6),
                    Some(3),
                    Some(1),
                    Some(5),
                ] as &[_],
            ),
            (
                (&[Some(4), Some(2), None, Some(3), Some(1)], 1, 3),
                &[Some(4), Some(2), None, Some(1), Some(1), Some(3), None, None, Some(1)],
            ),
        ];

        for ((root, val, depth), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::add_one_row(test_utilities::make_tree(root.iter().copied()), val, depth),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
