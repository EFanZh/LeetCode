pub mod iterative;

pub trait Solution {
    fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4] as &[_], &[2, 4, 4, 4] as &[_]),
            (&[1, 1, 2, 3], &[1, 3, 3]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::decompress_rl_elist(nums.to_vec()), expected);
        }
    }
}
