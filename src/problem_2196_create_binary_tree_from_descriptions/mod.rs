use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod hash_map;

pub trait Solution {
    fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[20, 15, 1], [20, 17, 0], [50, 20, 1], [50, 80, 0], [80, 19, 1]] as &[_],
                &[Some(50), Some(20), Some(80), Some(15), Some(17), Some(19)] as &[_],
            ),
            (
                &[[1, 2, 1], [2, 3, 0], [3, 4, 1]],
                &[Some(1), Some(2), None, None, Some(3), Some(4)],
            ),
        ];

        for (descriptions, expected) in test_cases {
            assert_eq!(
                S::create_binary_tree(descriptions.iter().map(Vec::from).collect()),
                test_utilities::make_tree(expected.iter().copied()),
            );
        }
    }
}
