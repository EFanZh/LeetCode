use crate::data_structures::ListNode;

pub mod two_passes;

pub trait Solution {
    fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 2, 3] as &[_], 5),
                &[&[1] as &[_], &[2], &[3], &[], &[]] as &[&[_]],
            ),
            (
                (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3),
                &[&[1, 2, 3, 4], &[5, 6, 7], &[8, 9, 10]],
            ),
        ];

        for ((head, k), expected) in test_cases {
            assert_eq!(
                S::split_list_to_parts(test_utilities::make_list(head.iter().copied()), k)
                    .iter()
                    .map(|list| test_utilities::iter_list(list).copied().collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                expected
            );
        }
    }
}
