pub mod greedy;

pub trait Solution {
    fn min_pair_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 5, 2, 3] as &[_], 7), (&[3, 5, 4, 2, 4, 6], 8)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_pair_sum(nums.to_vec()), expected);
        }
    }
}
