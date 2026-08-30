pub mod kmp;

pub trait Solution {
    fn count_matching_subarrays(nums: Vec<i32>, divisors: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5, 6] as &[_], &[1, 1] as &[_]), 4),
            ((&[1, 4, 4, 1, 3, 5, 5, 3], &[1, 0, -1]), 2),
        ];

        for ((nums, divisors), expected) in test_cases {
            assert_eq!(S::count_matching_subarrays(nums.to_vec(), divisors.to_vec()), expected);
        }
    }
}
