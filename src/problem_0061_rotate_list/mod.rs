use crate::data_structures::ListNode;

pub mod measure_length;

pub trait Solution {
    fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 2), &[4, 5, 1, 2, 3] as &[_]),
            ((&[0, 1, 2], 4), &[2, 0, 1]),
            ((&[], 0), &[]),
        ];

        for ((head, k), expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(&S::rotate_right(test_utilities::make_list(head.iter().copied()), k))
                    .copied()
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }
}
