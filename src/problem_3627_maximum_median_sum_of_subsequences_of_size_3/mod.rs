pub mod greedy;

pub trait Solution {
    fn maximum_median_sum(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 1, 3, 2, 1, 3] as &[_], 5), (&[1, 1, 10, 10, 10, 10], 20)];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_median_sum(nums.to_vec()), expected);
        }
    }
}
