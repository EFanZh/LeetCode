pub mod iterative;

pub trait Solution {
    fn peak_valley_difference(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 5, 2, 4, 1, 3, 0] as &[_], 9),
            (&[-5, -6, -1, -2, 5, 3, 2], 12),
            (&[6, 5, 2, 5, 5, 3, 6], -5),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::peak_valley_difference(nums.to_vec()), expected);
        }
    }
}
