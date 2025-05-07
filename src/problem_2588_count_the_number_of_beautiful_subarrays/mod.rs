pub mod hash_map;

pub trait Solution {
    fn beautiful_subarrays(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 3, 1, 2, 4] as &[_], 2), (&[1, 10, 4], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::beautiful_subarrays(nums.to_vec()), expected);
        }
    }
}
