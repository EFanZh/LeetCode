use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(1), Some(2)] as &[_],
                &[&["", "1", ""] as &[_], &["2", "", ""]] as &[&[_]],
            ),
            (
                &[Some(1), Some(2), Some(3), None, Some(4)],
                &[
                    &["", "", "", "1", "", "", ""],
                    &["", "2", "", "", "", "3", ""],
                    &["", "", "4", "", "", "", ""],
                ],
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(S::print_tree(test_utilities::make_tree(root.iter().copied())), expected);
        }
    }
}
