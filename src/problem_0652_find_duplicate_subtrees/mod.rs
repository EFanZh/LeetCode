use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod cached_hash_values;
pub mod hash_map;

pub trait Solution {
    fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    None,
                    Some(2),
                    Some(4),
                    None,
                    None,
                    Some(4),
                ] as &[_],
                &[&[Some(2), Some(4)] as &[_], &[Some(4)]] as &[&[_]],
            ),
            (&[Some(2), Some(1), Some(1)], &[&[Some(1)]]),
            (
                &[Some(2), Some(2), Some(2), Some(3), None, Some(3), None],
                &[&[Some(2), Some(3)], &[Some(3)]],
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(
                    S::find_duplicate_subtrees(test_utilities::make_tree(root.iter().copied()))
                        .into_iter()
                        .map(|root| test_utilities::iter_tree(root).collect::<Vec<_>>())
                        .collect::<Vec<_>>()
                ),
                expected
            );
        }
    }
}
