use super::data_structures::ListNode;

pub mod reverse_output;

pub trait Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[7, 2, 4, 3] as &[_], &[5, 6, 4] as &[_]), &[7, 8, 0, 7] as &[_]),
            ((&[5, 6, 4], &[7, 2, 4, 3]), &[7, 8, 0, 7]),
            ((&[9, 9], &[9]), &[1, 0, 8]),
        ];

        for ((l1, l2), expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::iter_list(&S::add_two_numbers(
                    test_utilities::make_list(l1.iter().copied()),
                    test_utilities::make_list(l2.iter().copied())
                ))
                .copied()
                .collect::<Box<_>>()
                .as_ref(),
                expected
            );
        }
    }
}
