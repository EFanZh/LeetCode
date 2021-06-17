use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(3), Some(9), Some(20), None, Some(15), Some(7)] as &[_],
                &[3.0, 14.5, 11.0] as &[_],
            ),
            (&[Some(3), Some(9), Some(20), Some(15), Some(7)], &[3.0, 14.5, 11.0]),
            (
                &[Some(0x7FFF_FFFF), Some(0x7FFF_FFFF), Some(0x7FFF_FFFF)],
                &[2_147_483_647.0, 2_147_483_647.0],
            ),
            (&[], &[]),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::average_of_levels(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
