#![allow(clippy::upper_case_acronyms)]

use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;

pub trait BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self;
    fn next(&mut self) -> i32;
    fn has_next(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::BSTIterator;
    use crate::test_utilities;

    enum Operation {
        Next(i32),
        HasNext(bool),
    }

    pub fn run<I: BSTIterator>() {
        use Operation::{HasNext, Next};

        let test_cases = [(
            &[Some(7), Some(3), Some(15), None, None, Some(9), Some(20)] as &[_],
            &[
                Next(3),
                Next(7),
                HasNext(true),
                Next(9),
                HasNext(true),
                Next(15),
                HasNext(true),
                Next(20),
                HasNext(false),
            ] as &[_],
        )];

        for (tree, operations) in test_cases {
            let mut iterator = I::new(test_utilities::make_tree(tree.iter().copied()));

            for operation in operations {
                match operation {
                    Next(expected) => assert_eq!(iterator.next(), *expected),
                    HasNext(expected) => assert_eq!(iterator.has_next(), *expected),
                }
            }
        }
    }
}
