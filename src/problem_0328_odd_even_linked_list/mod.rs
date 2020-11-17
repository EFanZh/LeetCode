use super::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5] as &[_], &[1, 3, 5, 2, 4] as &[_]),
            (&[2, 1, 3, 5, 6, 4, 7], &[2, 3, 6, 7, 1, 5, 4]),
        ];

        for (head, expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::iter_list(&S::odd_even_list(test_utilities::make_list(head.iter().copied())))
                    .copied()
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }
}
