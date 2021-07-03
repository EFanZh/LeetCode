use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 2, 4), &[1, 4, 3, 2, 5] as &[_]),
            ((&[1, 2, 3, 4, 5], 3, 4), &[1, 2, 4, 3, 5]),
            ((&[5], 1, 1), &[5]),
        ];

        for ((head, m, n), expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(&S::reverse_between(
                    test_utilities::make_list(head.iter().copied()),
                    m,
                    n
                ))
                .copied()
                .collect::<Box<_>>()
                .as_ref(),
                expected
            );
        }
    }
}
