use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;

pub trait Solution {
    fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] as &[_],
                &[&[15, 7] as &[_], &[9, 20], &[3]] as &[&[_]],
            ),
            (&[Some(1)], &[&[1]]),
            (&[], &[]),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::level_order_bottom(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
