pub mod greedy;

pub trait Solution {
    fn min_increment_for_unique(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 2] as &[_], 1), (&[3, 2, 1, 2, 1, 7], 6)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_increment_for_unique(nums.to_vec()), expected);
        }
    }
}
