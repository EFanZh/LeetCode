use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool;
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
                        Some(1),
                        Some(2),
                        Some(3),
                        Some(4),
                        Some(5),
                        Some(6),
                        Some(7),
                        Some(8),
                        Some(9),
                        Some(10),
                        Some(11),
                    ] as &[_],
                    11,
                    3,
                ),
                true,
            ),
            ((&[Some(1), Some(2), Some(3)], 3, 1), false),
        ];

        for ((root, n, x), expected) in test_cases {
            assert_eq!(
                S::btree_game_winning_move(test_utilities::make_tree(root.iter().copied()), n, x),
                expected,
            );
        }
    }
}
