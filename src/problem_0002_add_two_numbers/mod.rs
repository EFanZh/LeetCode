use crate::data_structures::ListNode;

pub mod simple;

pub trait Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 4, 3] as &[_], &[5, 6, 4] as &[_]), &[7, 0, 8] as &[_]),
            ((&[0], &[0]), &[0]),
            ((&[9, 9, 9, 9, 9, 9, 9], &[9, 9, 9, 9]), &[8, 9, 9, 9, 0, 0, 0, 1]),
            ((&[9, 9, 9, 9], &[9, 9, 9, 9, 9, 9, 9]), &[8, 9, 9, 9, 0, 0, 0, 1]),
        ];

        for ((l1, l2), expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(S::add_two_numbers(
                    test_utilities::make_list(l1.iter().copied()),
                    test_utilities::make_list(l2.iter().copied())
                ))
                .collect::<Box<_>>()
                .as_ref(),
                expected,
            );
        }
    }
}
