use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)] as &[_], 110),
            (
                &[Some(1), None, Some(2), Some(3), Some(4), None, None, Some(5), Some(6)],
                90,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::max_product(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
