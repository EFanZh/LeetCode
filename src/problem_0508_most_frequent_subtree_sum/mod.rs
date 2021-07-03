use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(5), Some(2), Some(-3)] as &[_], &[-3, 2, 4] as &[_]),
            (&[Some(5), Some(2), Some(-5)], &[2]),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::find_frequent_tree_sum(test_utilities::make_tree(
                    root.iter().copied()
                ))),
                expected
            );
        }
    }
}
