use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5] as &[_], &[3, 4, 5] as &[_]),
            (&[1, 2, 3, 4, 5, 6], &[4, 5, 6]),
        ];

        for (head, expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(&S::middle_node(test_utilities::make_list(head.iter().copied())))
                    .copied()
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }
}
