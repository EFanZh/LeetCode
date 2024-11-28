use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 3, 1, 0, 4, 5, 2, 0], &[4, 11] as &[_]),
            (&[0, 1, 0, 3, 0, 2, 2, 0], &[1, 3, 4]),
        ];

        for (head, expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(S::merge_nodes(test_utilities::make_list(head.iter().copied())))
                    .collect::<Vec<_>>(),
                expected,
            );
        }
    }
}
