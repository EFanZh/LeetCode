pub mod iterative;

pub trait Solution {
    fn check_prime_frequency(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5, 4] as &[_], true),
            (&[1, 2, 3, 4, 5], false),
            (&[2, 2, 2, 4, 4], true),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::check_prime_frequency(nums.to_vec()), expected);
        }
    }
}
