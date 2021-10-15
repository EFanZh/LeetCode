use crate::data_structures::ListNode;

pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[0, 1, 2, 3] as &[_], &[0, 1, 3] as &[_]), 2),
            ((&[0, 1, 2, 3, 4], &[0, 3, 1, 4]), 2),
            ((&[3, 4, 0, 2, 1], &[4]), 1),
        ];

        for ((head, nums), expected) in test_cases {
            assert_eq!(
                S::num_components(test_utilities::make_list(head.iter().copied()), nums.to_vec()),
                expected
            );
        }
    }
}
