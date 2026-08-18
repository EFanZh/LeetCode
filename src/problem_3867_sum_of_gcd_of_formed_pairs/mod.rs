pub mod brute_force;

pub trait Solution {
    fn gcd_sum(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 6, 4] as &[_], 2), (&[3, 6, 2, 8], 5)];

        for (nums, expected) in test_cases {
            assert_eq!(S::gcd_sum(nums.to_vec()), expected);
        }
    }
}
