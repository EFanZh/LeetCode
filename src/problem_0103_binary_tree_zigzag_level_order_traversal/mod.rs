use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;

pub trait Solution {
    fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] as &[_],
                &[&[3] as &[_], &[20, 9], &[15, 7]] as &[&[_]],
            ),
            (&[Some(1)], &[&[1]]),
            (&[], &[]),
            ((&[Some(1), Some(2)]), &[&[1], &[2]]),
        ];

        for (root, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::zigzag_level_order(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
