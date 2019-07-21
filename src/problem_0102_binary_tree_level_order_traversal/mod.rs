use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;

pub trait Solution {
    fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities::make_tree;
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = vec![(
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            vec![vec![3], vec![9, 20], vec![15, 7]],
        )];

        for (serialized_tree, expected) in test_cases {
            assert_eq!(S::level_order(make_tree(serialized_tree)), expected);
        }
    }
}
