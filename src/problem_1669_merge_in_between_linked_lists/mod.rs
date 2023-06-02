use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[0, 1, 2, 3, 4, 5] as &[_],
                    3,
                    4,
                    &[1_000_000, 1_000_001, 1_000_002] as &[_],
                ),
                &[0, 1, 2, 1_000_000, 1_000_001, 1_000_002, 5] as &[_],
            ),
            (
                (
                    &[0, 1, 2, 3, 4, 5, 6],
                    2,
                    5,
                    &[1_000_000, 1_000_001, 1_000_002, 1_000_003, 1_000_004],
                ),
                &[0, 1, 1_000_000, 1_000_001, 1_000_002, 1_000_003, 1_000_004, 6],
            ),
        ];

        for ((list1, a, b, list2), expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(&S::merge_in_between(
                    test_utilities::make_list(list1.iter().copied()),
                    a,
                    b,
                    test_utilities::make_list(list2.iter().copied())
                ))
                .copied()
                .collect::<Vec<_>>(),
                expected
            );
        }
    }
}
