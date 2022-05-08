use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;
pub mod stack;

pub trait Solution {
    fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                "1-2--3--4-5--6--7",
                &[Some(1), Some(2), Some(5), Some(3), Some(4), Some(6), Some(7)] as &[_],
            ),
            (
                "1-2--3---4-5--6---7",
                &[
                    Some(1),
                    Some(2),
                    Some(5),
                    Some(3),
                    None,
                    Some(6),
                    None,
                    Some(4),
                    None,
                    Some(7),
                ],
            ),
            (
                "1-401--349---90--88",
                &[Some(1), Some(401), None, Some(349), Some(88), Some(90)],
            ),
        ];

        for (traversal, expected) in test_cases {
            assert_eq!(
                S::recover_from_preorder(traversal.to_string()),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
