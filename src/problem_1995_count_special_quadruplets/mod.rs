pub mod hash_map;

pub trait Solution {
    fn count_quadruplets(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 6] as &[_], 1), (&[3, 3, 6, 4, 5], 0), (&[1, 1, 1, 3, 5], 4)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_quadruplets(nums.to_vec()), expected);
        }
    }
}
