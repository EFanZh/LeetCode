use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod dynamic_programming;

pub trait Solution {
    fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, &[&[Some(0)] as &[_]] as &[&[_]]),
            (2, &[]),
            (3, &[&[Some(0), Some(0), Some(0)]]),
            (4, &[]),
            (
                5,
                &[
                    &[Some(0), Some(0), Some(0), None, None, Some(0), Some(0)],
                    &[Some(0), Some(0), Some(0), Some(0), Some(0)],
                ],
            ),
            (6, &[]),
            (
                7,
                &[
                    &[
                        Some(0),
                        Some(0),
                        Some(0),
                        None,
                        None,
                        Some(0),
                        Some(0),
                        None,
                        None,
                        Some(0),
                        Some(0),
                    ],
                    &[
                        Some(0),
                        Some(0),
                        Some(0),
                        None,
                        None,
                        Some(0),
                        Some(0),
                        Some(0),
                        Some(0),
                    ],
                    &[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)],
                    &[
                        Some(0),
                        Some(0),
                        Some(0),
                        Some(0),
                        Some(0),
                        None,
                        None,
                        None,
                        None,
                        Some(0),
                        Some(0),
                    ],
                    &[
                        Some(0),
                        Some(0),
                        Some(0),
                        Some(0),
                        Some(0),
                        None,
                        None,
                        Some(0),
                        Some(0),
                    ],
                ],
            ),
        ];

        for (n, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted_by(S::all_possible_fbt(n), test_utilities::compare_tree),
                expected
                    .iter()
                    .map(|tree| test_utilities::make_tree(tree.iter().copied()))
                    .collect::<Box<_>>()
                    .as_ref()
            );
        }
    }
}
