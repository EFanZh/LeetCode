pub mod dynamic_programming;

pub trait Solution {
    fn longest_arith_seq_length(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 6, 9, 12] as &[_], 4),
            (&[9, 4, 7, 2, 10], 3),
            (&[20, 1, 15, 3, 10, 5, 8], 4),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::longest_arith_seq_length(nums.to_vec()), expected);
        }
    }
}
