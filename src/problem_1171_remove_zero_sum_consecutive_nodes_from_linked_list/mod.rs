use crate::data_structures::ListNode;

pub mod greedy;

pub trait Solution {
    fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, -3, 3, 1] as &[_], &[&[3, 1] as &[_], &[1, 2, 1]] as &[&[_]]),
            (&[1, 2, 3, -3, 4], &[&[1, 2, 4]]),
            (&[1, 2, 3, -3, -2], &[&[1]]),
            (&[1, 3, 2, -3, -2, 5, 100, -100, 1], &[&[1, 5, 1], &[1, 3, 2, 1]]),
        ];

        for (head, expected) in test_cases {
            assert!(expected.contains(
                &test_utilities::iter_list(&S::remove_zero_sum_sublists(test_utilities::make_list(
                    head.iter().copied()
                )))
                .copied()
                .collect::<Vec<_>>()
                .as_slice(),
            ));
        }
    }
}
