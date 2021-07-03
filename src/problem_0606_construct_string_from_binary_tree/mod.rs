use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1), Some(2), Some(3), Some(4)] as &[_], "1(2(4))(3)"),
            (&[Some(1), Some(2), Some(3), None, Some(4)], "1(2()(4))(3)"),
            (&[], ""),
        ];

        for (t, expected) in test_cases {
            assert_eq!(S::tree2str(test_utilities::make_tree(t.iter().copied())), expected);
        }
    }
}
