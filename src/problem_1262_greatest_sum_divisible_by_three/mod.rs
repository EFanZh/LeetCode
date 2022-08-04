pub mod dynamic_programming;
pub mod dynamic_programming_2;
pub mod dynamic_programming_3;
pub mod dynamic_programming_4;

pub trait Solution {
    fn max_sum_div_three(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 6, 5, 1, 8] as &[_], 18),
            (&[4], 0),
            (&[1, 2, 3, 4, 4], 12),
            (&[4, 1, 5, 3, 1], 12),
            (&[3, 1, 2], 6),
            (&[7, 3, 7, 4, 2, 2, 7, 5], 33),
            (&[2, 6, 2, 2, 7], 15),
            (&[3, 1, 3], 6),
            (&[3, 2, 3], 6),
            (&[3, 6, 9], 18),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_sum_div_three(nums.to_vec()), expected);
        }
    }
}
