pub mod iterative;

pub trait Solution {
    fn score_difference(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3] as &[_], 0), (&[2, 4, 2, 1, 2, 1], 4), (&[1], -1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::score_difference(nums.to_vec()), expected);
        }
    }
}
