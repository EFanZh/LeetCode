pub mod iterative;

pub trait Solution {
    fn subset_xor_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3] as &[_], 6), (&[5, 1, 6], 28), (&[3, 4, 5, 6, 7, 8], 480)];

        for (nums, expected) in test_cases {
            assert_eq!(S::subset_xor_sum(nums.to_vec()), expected);
        }
    }
}
