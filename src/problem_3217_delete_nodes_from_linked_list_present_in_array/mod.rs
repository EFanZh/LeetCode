use crate::data_structures::ListNode;

pub mod bit_flags;

pub trait Solution {
    fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], &[1, 2, 3, 4, 5] as &[_]), &[4, 5] as &[_]),
            ((&[1], &[1, 2, 1, 2, 1, 2]), &[2, 2, 2]),
            ((&[5], &[1, 2, 3, 4]), &[1, 2, 3, 4]),
        ];

        for ((nums, head), expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(S::modified_list(
                    nums.to_vec(),
                    test_utilities::make_list(head.iter().copied())
                ))
                .collect::<Vec<_>>(),
                expected,
            );
        }
    }
}
