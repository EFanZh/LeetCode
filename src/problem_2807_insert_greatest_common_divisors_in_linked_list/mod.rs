use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[18, 6, 10, 3] as &[_], &[18, 6, 6, 2, 10, 1, 3] as &[_]),
            (&[7], &[7]),
        ];

        for (head, expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(S::insert_greatest_common_divisors(test_utilities::make_list(
                    head.iter().copied()
                )))
                .collect::<Vec<_>>(),
                expected,
            );
        }
    }
}
