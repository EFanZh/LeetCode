pub mod hash_map;

pub trait Solution {
    fn count_nice_pairs(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[42, 11, 1, 97] as &[_], 2), (&[13, 10, 35, 24, 76], 4)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_nice_pairs(nums.to_vec()), expected);
        }
    }
}
