pub mod dynamic_programming;

pub trait Solution {
    fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], &[3, 2, 1] as &[_]), 14),
            ((&[-5, -3, -3, -2, 7, 1], &[-10, -5, 3, 4, 6]), 102),
        ];

        for ((nums, multipliers), expected) in test_cases {
            assert_eq!(S::maximum_score(nums.to_vec(), multipliers.to_vec()), expected);
        }
    }
}
