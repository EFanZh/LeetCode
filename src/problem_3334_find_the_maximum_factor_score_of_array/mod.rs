pub mod iterative;

pub trait Solution {
    fn max_score(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 4, 8, 16] as &[_], 64), (&[1, 2, 3, 4, 5], 60), (&[3], 9)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_score(nums.to_vec()), expected);
        }
    }
}
