pub mod iterative;

pub trait Solution {
    fn min_max_game(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 5, 2, 4, 8, 2, 2] as &[_], 1), (&[3], 3)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_max_game(nums.to_vec()), expected);
        }
    }
}
