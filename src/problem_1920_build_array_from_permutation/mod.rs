pub mod utilize_higher_bits;

pub trait Solution {
    fn build_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 2, 1, 5, 3, 4] as &[_], &[0, 1, 2, 4, 5, 3] as &[_]),
            (&[5, 0, 1, 2, 3, 4], &[4, 5, 0, 1, 2, 3]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::build_array(nums.to_vec()), expected);
        }
    }
}
