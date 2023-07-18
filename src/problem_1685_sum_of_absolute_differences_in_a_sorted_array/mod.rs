pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 3, 5] as &[_], &[4, 3, 5] as &[_]),
            (&[1, 4, 6, 8, 10], &[24, 15, 13, 15, 21]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::get_sum_absolute_differences(nums.to_vec()), expected);
        }
    }
}
