use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 3, 4, 7, 1, 2, 6] as &[_], &[1, 3, 4, 1, 2, 6] as &[_]),
            (&[1, 2, 3, 4], &[1, 2, 4]),
            (&[2, 1], &[2]),
        ];

        for (head, expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(S::delete_middle(test_utilities::make_list(head.iter().copied())))
                    .collect::<Vec<_>>(),
                expected,
            );
        }
    }
}
