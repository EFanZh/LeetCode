use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod count_edge_flow;

pub trait Solution {
    fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(3), Some(0), Some(0)] as &[_], 2),
            (&[Some(0), Some(3), Some(0)], 3),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::distribute_coins(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
