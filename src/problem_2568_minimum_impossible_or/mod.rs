pub mod greedy;

pub trait Solution {
    fn min_impossible_or(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 1] as &[_], 4), (&[5, 3, 2], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_impossible_or(nums.to_vec()), expected);
        }
    }
}
