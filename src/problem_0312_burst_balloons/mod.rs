pub mod dynamic_programming;

pub trait Solution {
    fn max_coins(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 1, 5, 8] as &[_], 167)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::max_coins(nums.to_vec()), expected);
        }
    }
}
