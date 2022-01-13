pub mod count_add_and_sub_separately;

pub trait Solution {
    fn sum_subseq_widths(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 1, 3] as &[_], 6),
            (&[2], 0),
            (
                &[
                    5, 69, 89, 92, 31, 16, 25, 45, 63, 40, 16, 56, 24, 40, 75, 82, 40, 12, 50, 62, 92, 44, 67, 38, 92,
                    22, 91, 24, 26, 21, 100, 42, 23, 56, 64, 43, 95, 76, 84, 79, 89, 4, 16, 94, 16, 77, 92, 9, 30, 13,
                ],
                857_876_214,
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::sum_subseq_widths(nums.to_vec()), expected);
        }
    }
}
