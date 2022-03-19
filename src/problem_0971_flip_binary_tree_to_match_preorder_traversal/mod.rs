use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    #[allow(clippy::too_many_lines)]
    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[Some(1), Some(2)] as &[_], &[2, 1] as &[_]), &[-1] as &[_]),
            ((&[Some(1), Some(2), Some(3)], &[1, 3, 2]), &[1]),
            ((&[Some(1), Some(2), Some(3)], &[1, 2, 3]), &[]),
            ((&[Some(1), Some(2), None, Some(3)], &[1, 3, 2]), &[-1]),
            (
                (
                    &[
                        Some(15),
                        Some(26),
                        Some(40),
                        Some(25),
                        Some(38),
                        Some(29),
                        Some(6),
                        Some(48),
                        Some(1),
                        Some(16),
                        Some(8),
                        Some(18),
                        Some(27),
                        Some(21),
                        Some(19),
                        Some(31),
                        Some(39),
                        Some(46),
                        None,
                        Some(49),
                        Some(45),
                        None,
                        Some(22),
                        Some(34),
                        Some(41),
                        Some(42),
                        None,
                        Some(9),
                        None,
                        Some(2),
                        Some(4),
                        None,
                        None,
                        None,
                        Some(17),
                        None,
                        None,
                        None,
                        None,
                        Some(36),
                        Some(24),
                        None,
                        Some(33),
                        None,
                        Some(30),
                        None,
                        None,
                        Some(23),
                        Some(37),
                        None,
                        None,
                        Some(5),
                        Some(12),
                        None,
                        Some(10),
                        Some(7),
                        None,
                        None,
                        None,
                        Some(13),
                        None,
                        None,
                        None,
                        Some(11),
                        Some(43),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(50),
                        Some(35),
                        Some(3),
                        Some(32),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(47),
                        None,
                        Some(28),
                        Some(44),
                        Some(14),
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(20),
                    ],
                    &[
                        15, 40, 6, 21, 9, 19, 2, 12, 50, 47, 35, 28, 20, 5, 4, 10, 32, 3, 14, 44, 29, 18, 34, 7, 11,
                        43, 41, 27, 42, 37, 23, 26, 38, 16, 45, 24, 13, 36, 49, 8, 22, 33, 25, 1, 46, 48, 31, 39, 17,
                        30,
                    ],
                ),
                &[-1],
            ),
        ];

        for ((root, voyage), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::flip_match_voyage(
                    test_utilities::make_tree(root.iter().copied()),
                    voyage.to_vec()
                )),
                expected
            );
        }
    }
}
