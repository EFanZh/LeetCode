use super::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn reorder_list(head: &mut Option<Box<ListNode>>);
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4] as &[_], &[1, 4, 2, 3] as &[_]),
            (&[1, 2, 3, 4, 5], &[1, 5, 2, 4, 3]),
        ];

        for (head, expected) in test_cases {
            let mut head = test_utilities::make_list(head.iter().copied());

            S::reorder_list(&mut head);

            assert_eq!(
                test_utilities::iter_list(&head).copied().collect::<Box<_>>().as_ref(),
                expected
            );
        }
    }
}
