pub mod hash_map;

pub trait Solution {
    fn unequal_triplets(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 4, 2, 4, 3] as &[_], 3), (&[1, 1, 1, 1, 1], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::unequal_triplets(nums.to_vec()), expected);
        }
    }
}
