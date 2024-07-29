use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod bfs;

pub trait Solution {
    fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    &[Some(2), Some(1)] as &[_],
                    &[Some(3), Some(2), Some(5)],
                    &[Some(5), Some(4)],
                ] as &[&[_]],
                &[Some(3), Some(2), Some(5), Some(1), None, Some(4)] as &[_],
            ),
            (&[&[Some(5), Some(3), Some(8)], &[Some(3), Some(2), Some(6)]], &[]),
            (&[&[Some(5), Some(4)], &[Some(3)]], &[]),
            (
                &[
                    &[Some(4), Some(1)],
                    &[Some(1), None, Some(2)],
                    &[Some(2), None, Some(3)],
                ],
                &[Some(4), Some(1), None, None, Some(2), None, Some(3)],
            ),
            (
                &[&[Some(1), None, Some(3)], &[Some(3), Some(1)], &[Some(4), Some(2)]],
                &[],
            ),
            (
                &[
                    &[Some(8), None, Some(9)],
                    &[Some(4), None, Some(5)],
                    &[Some(9), Some(4)],
                ],
                &[],
            ),
            (&[&[Some(3), Some(1)], &[Some(1)], &[Some(2), Some(1)]], &[]),
        ];

        for (trees, expected) in test_cases {
            assert_eq!(
                S::can_merge(
                    trees
                        .iter()
                        .map(|tree| test_utilities::make_tree(tree.iter().copied()))
                        .collect(),
                ),
                test_utilities::make_tree(expected.iter().copied()),
            );
        }
    }
}
