use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(4), Some(2), Some(6), Some(1), Some(3)] as &[_], 1),
            (&[Some(1), Some(0), Some(48), None, None, Some(12), Some(49)], 1),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::get_minimum_difference(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
