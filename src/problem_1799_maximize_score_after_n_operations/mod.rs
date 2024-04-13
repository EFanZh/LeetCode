pub mod dynamic_programming;

pub trait Solution {
    fn max_score(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2] as &[_], 1), (&[3, 4, 6, 8], 11), (&[1, 2, 3, 4, 5, 6], 14)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_score(nums.to_vec()), expected);
        }
    }
}
