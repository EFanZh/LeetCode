use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;
pub mod recursive_2;

pub trait Solution {
    fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                3,
                &[
                    &[Some(1), None, Some(2), None, Some(3)],
                    &[Some(1), None, Some(3), Some(2)] as &[_],
                    &[Some(2), Some(1), Some(3)],
                    &[Some(3), Some(1), None, None, Some(2)],
                    &[Some(3), Some(2), None, Some(1)],
                ] as &[&[_]],
            ),
            (0, &[]),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::generate_trees(n),
                expected
                    .iter()
                    .map(|tree| test_utilities::make_tree(tree.iter().copied()))
                    .collect::<Box<_>>()
                    .as_ref()
            );
        }
    }
}
