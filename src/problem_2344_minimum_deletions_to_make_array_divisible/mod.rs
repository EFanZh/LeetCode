pub mod gcd;

pub trait Solution {
    fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 3, 2, 4, 3] as &[_], &[9, 6, 9, 3, 15] as &[_]), 2),
            ((&[4, 3, 6], &[8, 2, 6, 10]), -1),
        ];

        for ((nums, nums_divide), expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec(), nums_divide.to_vec()), expected);
        }
    }
}
