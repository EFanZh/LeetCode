use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod binary_search;
pub mod binary_search_2;

pub trait Solution {
    fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[
                        Some(6),
                        Some(2),
                        Some(13),
                        Some(1),
                        Some(4),
                        Some(9),
                        Some(15),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(14),
                    ] as &[_],
                    &[2, 5, 16] as &[_],
                ),
                &[[2, 2], [4, 6], [15, -1]] as &[_],
            ),
            ((&[Some(4), None, Some(9)], &[3]), &[[-1, 4]]),
        ];

        for ((root, queries), expected) in test_cases {
            assert_eq!(
                S::closest_nodes(test_utilities::make_tree(root.iter().copied()), queries.to_vec()),
                expected,
            );
        }
    }
}
