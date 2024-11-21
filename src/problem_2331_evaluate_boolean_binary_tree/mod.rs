use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    type TestCase<'a> = (&'a [Option<i32>], bool);

    const EXTRA_TEST_CASE_1: TestCase = (
        &[
            Some(3),
            Some(3),
            Some(2),
            Some(0),
            Some(0),
            Some(3),
            Some(2),
            None,
            None,
            None,
            None,
            Some(1),
            Some(3),
            Some(1),
            Some(1),
            None,
            None,
            Some(2),
            Some(1),
            None,
            None,
            None,
            None,
            Some(1),
            Some(1),
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        false,
    );

    const EXTRA_TEST_CASE_2: TestCase = (
        &[
            Some(3),
            Some(2),
            Some(3),
            Some(3),
            Some(3),
            Some(3),
            Some(3),
            Some(2),
            Some(2),
            Some(2),
            Some(1),
            Some(2),
            Some(0),
            Some(0),
            Some(3),
            Some(0),
            Some(2),
            Some(0),
            Some(1),
            Some(0),
            Some(0),
            None,
            None,
            Some(0),
            Some(1),
            None,
            None,
            None,
            None,
            Some(1),
            Some(1),
            None,
            None,
            Some(3),
            Some(0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(1),
            Some(0),
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        false,
    );

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(2), Some(1), Some(3), None, None, Some(0), Some(1)] as &[_], true),
            (&[Some(0)], false),
            EXTRA_TEST_CASE_1,
            EXTRA_TEST_CASE_2,
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::evaluate_tree(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
