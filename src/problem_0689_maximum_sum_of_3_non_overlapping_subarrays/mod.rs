pub mod dynamic_programming;

pub trait Solution {
    fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 1, 2, 6, 7, 5, 1] as &[_], 2), [0, 3, 5]),
            ((&[1, 2, 1, 2, 1, 2, 1, 2, 1], 2), [0, 2, 4]),
            ((&[7, 13, 20, 19, 19, 2, 10, 1, 1, 19], 3), [1, 4, 7]),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::max_sum_of_three_subarrays(nums.to_vec(), k), expected);
        }
    }
}
