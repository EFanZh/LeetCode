pub mod mathematical;

pub trait Solution {
    fn unique_xor_triplets(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2] as &[_], 2), (&[3, 1, 2], 4), (&[3, 1, 2, 4], 8)];

        for (nums, expected) in test_cases {
            assert_eq!(S::unique_xor_triplets(nums.to_vec()), expected);
        }
    }
}
