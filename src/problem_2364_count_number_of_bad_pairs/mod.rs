pub mod iterative;

pub trait Solution {
    fn count_bad_pairs(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 1, 3, 3] as &[_], 5), (&[1, 2, 3, 4, 5], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_bad_pairs(nums.to_vec()), expected);
        }
    }
}
