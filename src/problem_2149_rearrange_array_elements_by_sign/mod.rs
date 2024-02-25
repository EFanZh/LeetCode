pub mod iterative;

pub trait Solution {
    fn rearrange_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 1, -2, -5, 2, -4] as &[_], &[3, -2, 1, -5, 2, -4] as &[_]),
            (&[-1, 1], &[1, -1]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::rearrange_array(nums.to_vec()), expected);
        }
    }
}
