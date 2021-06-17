pub mod iterative;

pub trait Solution {
    fn total_hamming_distance(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 14, 2] as &[_], 6), (&[4, 14, 4], 4)];

        for (nums, expected) in test_cases {
            assert_eq!(S::total_hamming_distance(nums.to_vec()), expected);
        }
    }
}
