pub mod brute_force;

pub trait Solution {
    fn sum_four_divisors(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[21, 4, 7] as &[_], 32),
            (&[21, 21], 64),
            (&[1, 2, 3, 4, 5], 0),
            (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 45),
            (&[7286, 18704, 70773, 8224, 91675], 10932),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::sum_four_divisors(nums.to_vec()), expected);
        }
    }
}
