pub mod dynamic_programming;

pub trait Solution {
    fn longest_square_streak(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 3, 6, 16, 8, 2] as &[_], 3), (&[2, 3, 5, 6, 7], -1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::longest_square_streak(nums.to_vec()), expected);
        }
    }
}
