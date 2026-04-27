pub mod hash_map;

pub trait Solution {
    fn special_triplets(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[6, 3, 6] as &[_], 1), (&[0, 1, 0, 0], 1), (&[8, 4, 2, 8, 4], 2)];

        for (nums, expected) in test_cases {
            assert_eq!(S::special_triplets(nums.to_vec()), expected);
        }
    }
}
