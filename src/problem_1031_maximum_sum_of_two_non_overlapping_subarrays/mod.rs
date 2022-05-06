pub mod dynamic_programming;

pub trait Solution {
    fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[0, 6, 5, 2, 2, 5, 1, 9, 4] as &[_], 1, 2), 20),
            ((&[3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2), 29),
            ((&[2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3), 31),
        ];

        for ((nums, first_len, second_len), expected) in test_cases {
            assert_eq!(
                S::max_sum_two_no_overlap(nums.to_vec(), first_len, second_len),
                expected
            );
        }
    }
}
