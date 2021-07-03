use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;
pub mod recursive_2;
pub mod recursive_3;

pub trait Solution {
    fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(2), Some(1), Some(3)] as &[_], true),
            (&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)], false),
            (&[Some(-2_147_483_648), Some(-2_147_483_648)], false),
            (&[], true),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::is_valid_bst(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
