pub mod hash_set;

pub trait Solution {
    fn count_distinct_integers(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 13, 10, 12, 31] as &[_], 6), (&[2, 2, 2], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_distinct_integers(nums.to_vec()), expected);
        }
    }
}
