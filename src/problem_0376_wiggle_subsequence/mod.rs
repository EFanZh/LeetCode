pub mod greedy;

pub trait Solution {
    fn wiggle_max_length(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 1, 5, 8] as &[_], 167)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::wiggle_max_length(nums.to_vec()), expected);
        }
    }
}
