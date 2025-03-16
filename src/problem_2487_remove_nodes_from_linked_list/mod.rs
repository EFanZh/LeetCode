use crate::data_structures::ListNode;

pub mod monotonic_stack;

pub trait Solution {
    fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[5, 2, 13, 3, 8] as &[_], &[13, 8] as &[_]),
            (&[1, 1, 1, 1], &[1, 1, 1, 1]),
        ];

        for (head, expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(S::remove_nodes(test_utilities::make_list(head.iter().copied())))
                    .collect::<Vec<_>>(),
                expected,
            );
        }
    }
}
