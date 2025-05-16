pub mod greedy;

pub trait Solution {
    fn prime_sub_operation(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 9, 6, 10] as &[_], true),
            (&[6, 8, 11, 12], true),
            (&[5, 8, 3], false),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::prime_sub_operation(nums.to_vec()), expected);
        }
    }
}
