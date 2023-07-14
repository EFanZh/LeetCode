use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 2), &[1, 4, 3, 2, 5] as &[_]),
            (
                (&[7, 9, 6, 6, 7, 8, 3, 0, 9, 5] as &[_], 5),
                &[7, 9, 6, 6, 8, 7, 3, 0, 9, 5],
            ),
            ((&[1], 1), &[1]),
            ((&[100, 90], 2), &[90, 100]),
        ];

        for ((head, k), expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(&S::swap_nodes(test_utilities::make_list(head.iter().copied()), k))
                    .copied()
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }
}
