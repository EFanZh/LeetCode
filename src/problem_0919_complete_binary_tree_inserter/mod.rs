use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod queue;

pub trait CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self;
    fn insert(&mut self, val: i32) -> i32;
    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::CBTInserter;
    use crate::test_utilities;

    enum Operation<'a> {
        Insert(i32, i32),
        GetRoot(&'a [i32]),
    }

    pub fn run<S: CBTInserter>() {
        let test_cases = [
            (
                &[1, 2] as &[_],
                &[
                    Operation::Insert(3, 1),
                    Operation::Insert(4, 2),
                    Operation::GetRoot(&[1, 2, 3, 4]),
                ] as &[_],
            ),
            (&[1], &[Operation::Insert(2, 1), Operation::GetRoot(&[1, 2])]),
            (
                &[1, 2, 3, 4, 5, 6],
                &[
                    Operation::Insert(7, 3),
                    Operation::Insert(8, 4),
                    Operation::GetRoot(&[1, 2, 3, 4, 5, 6, 7, 8]),
                ],
            ),
        ];

        for (root, operations) in test_cases {
            let mut cbt_inserter = S::new(test_utilities::make_tree(root.iter().copied().map(Some)));

            for operation in operations {
                match *operation {
                    Operation::Insert(val, expected) => assert_eq!(cbt_inserter.insert(val), expected),
                    Operation::GetRoot(expected) => assert_eq!(
                        cbt_inserter.get_root(),
                        test_utilities::make_tree(expected.iter().copied().map(Some))
                    ),
                }
            }
        }
    }
}
