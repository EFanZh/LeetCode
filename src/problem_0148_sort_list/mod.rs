use crate::data_structures::ListNode;

pub mod merge_sort;

pub trait Solution {
    fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 2, 1, 3] as &[_], &[1, 2, 3, 4] as &[_]),
            (&[-1, 5, 3, 4, 0], &[-1, 0, 3, 4, 5]),
            (&[], &[]),
        ];

        for (head, expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(&S::sort_list(test_utilities::make_list(head.iter().copied())))
                    .copied()
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }
}
