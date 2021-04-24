use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), None, Some(2), Some(2)] as &[_], &[2] as &[_]),
            (&[Some(0)], &[0]),
            (&[Some(1), None, Some(2)], &[1, 2]),
        ];

        for (root, expected) in test_cases.iter().copied() {
            assert_eq!(S::find_mode(test_utilities::make_tree(root.iter().copied())), expected);
        }
    }
}
