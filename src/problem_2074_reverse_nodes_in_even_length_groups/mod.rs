use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[5, 2, 6, 3, 9, 1, 7, 3, 8, 4] as &[_],
                &[5, 6, 2, 3, 9, 1, 4, 8, 3, 7] as &[_],
            ),
            (&[1, 1, 0, 6], &[1, 0, 1, 6]),
            (&[1, 1, 0, 6, 5], &[1, 0, 1, 5, 6]),
            (&[2, 1], &[2, 1]),
        ];

        for (head, expected) in test_cases {
            assert_eq!(
                &test_utilities::iter_list(&S::reverse_even_length_groups(test_utilities::make_list(
                    head.iter().copied()
                )))
                .copied()
                .collect::<Vec<_>>(),
                expected,
            );
        }
    }
}
