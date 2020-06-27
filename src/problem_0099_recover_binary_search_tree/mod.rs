use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod morris_traversal;

pub trait Solution {
    fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>);
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(1), Some(3), None, None, Some(2)] as &[_],
                &[Some(3), Some(1), None, None, Some(2)] as &[_],
            ),
            (
                &[Some(3), Some(1), Some(4), None, None, Some(2)],
                &[Some(2), Some(1), Some(4), None, None, Some(3)],
            ),
        ];

        for (root, expected) in test_cases.iter().copied() {
            let mut root = test_utilities::make_tree(root.iter().copied());

            S::recover_tree(&mut root);

            assert_eq!(root, test_utilities::make_tree(expected.iter().copied()));
        }
    }
}
