pub mod dynamic_programming;

pub trait Solution {
    fn count_max_or_subsets(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 1] as &[_], 2), (&[2, 2, 2], 7), (&[3, 2, 1, 5], 6)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_max_or_subsets(nums.to_vec()), expected);
        }
    }
}
