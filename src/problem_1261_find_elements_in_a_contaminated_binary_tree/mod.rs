use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;

pub trait FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self;
    fn find(&self, target: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::FindElements;
    use crate::test_utilities;

    pub fn run<F: FindElements>() {
        let test_cases = [
            (&[Some(-1), None, Some(-1)] as &[_], &[(1, false), (2, true)] as &[_]),
            (
                &[Some(-1), Some(-1), Some(-1), Some(-1), Some(-1)],
                &[(1, true), (3, true), (5, false)],
            ),
            (
                &[Some(-1), None, Some(-1), Some(-1), None, Some(-1)],
                &[(2, true), (3, false), (4, false), (5, true)],
            ),
        ];

        for (root, expected) in test_cases {
            let find_elements = F::new(test_utilities::make_tree(root.iter().copied()));

            for &(target, expected) in expected {
                assert_eq!(find_elements.find(target), expected);
            }
        }
    }
}
