pub mod hash_set;

pub trait Solution {
    fn distinct_averages(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 1, 4, 0, 3, 5] as &[_], 2), (&[1, 100], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::distinct_averages(nums.to_vec()), expected);
        }
    }
}
