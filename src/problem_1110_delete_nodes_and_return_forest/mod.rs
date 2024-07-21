use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)] as &[_],
                    &[3, 5] as &[_],
                ),
                &[&[Some(1), Some(2), None, Some(4)] as &[_], &[Some(6)], &[Some(7)]] as &[&[_]],
            ),
            (
                (&[Some(1), Some(2), Some(4), None, Some(3)], &[3]),
                &[&[Some(1), Some(2), Some(4)]],
            ),
            (
                (&[Some(1), Some(2), Some(3), None, None, None, Some(4)], &[2, 1]),
                &[&[Some(3), None, Some(4)]],
            ),
        ];

        for ((root, to_delete), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted_by(
                    S::del_nodes(test_utilities::make_tree(root.iter().copied()), to_delete.to_vec()),
                    test_utilities::compare_tree,
                ),
                expected
                    .iter()
                    .map(|tree| test_utilities::make_tree(tree.iter().copied()))
                    .collect::<Vec<_>>()
            );
        }
    }
}
