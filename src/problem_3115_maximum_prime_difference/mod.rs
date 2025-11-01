pub mod greedy;

pub trait Solution {
    fn maximum_prime_difference(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 2, 9, 5, 3] as &[_], 3), (&[4, 8, 2, 8], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_prime_difference(nums.to_vec()), expected);
        }
    }
}
