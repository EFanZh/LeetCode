pub mod highest_bit_as_marker;
pub mod partition_by_bit;

pub trait Solution {
    fn find_error_nums(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 2, 4] as &[_], [2, 3]),
            (&[1, 1], [1, 2]),
            (&[1, 2, 2, 4], [2, 3]),
            (&[3, 3, 1], [3, 2]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_error_nums(nums.to_vec()), expected);
        }
    }
}
