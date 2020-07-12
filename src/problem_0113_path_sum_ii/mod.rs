use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            (
                &[
                    Some(5),
                    Some(4),
                    Some(8),
                    Some(11),
                    None,
                    Some(13),
                    Some(4),
                    Some(7),
                    Some(2),
                    None,
                    None,
                    Some(5),
                    Some(1),
                ] as &[_],
                22,
            ),
            &[&[5, 4, 11, 2] as &[_], &[5, 8, 4, 5]] as &[&[_]],
        )];

        for ((root, sum), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::path_sum(test_utilities::make_tree(root.iter().copied()), sum),
                expected
            );
        }
    }
}
