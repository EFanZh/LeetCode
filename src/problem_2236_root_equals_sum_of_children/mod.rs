use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod obvious;

pub trait Solution {
    fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [([10, 4, 6], true), ([5, 3, 1], false)];

        for (root, expected) in test_cases {
            assert_eq!(S::check_tree(test_utilities::make_tree(root.map(Some))), expected);
        }
    }
}
