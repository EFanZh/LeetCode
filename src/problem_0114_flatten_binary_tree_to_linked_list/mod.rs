use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>);
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)] as &[_],
                &[
                    Some(1),
                    None,
                    Some(2),
                    None,
                    Some(3),
                    None,
                    Some(4),
                    None,
                    Some(5),
                    None,
                    Some(6),
                ] as &[_],
            ),
            (&[], &[]),
            (&[Some(0)], &[Some(0)]),
        ];

        for (root, expected) in test_cases {
            let mut root = test_utilities::make_tree(root.iter().copied());

            S::flatten(&mut root);

            assert_eq!(root, test_utilities::make_tree(expected.iter().copied()));
        }
    }
}
